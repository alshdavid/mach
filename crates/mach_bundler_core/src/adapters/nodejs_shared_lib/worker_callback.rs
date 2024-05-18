use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

use crate::public::AdapterOutgoingRequest;
use crate::public::AdapterOutgoingResponse;
use crate::public::AdapterOutgoingResponsePing;
use crate::public::AdapterOutgoingResponseResolverLoadConfig;
use crate::public::AdapterOutgoingResponseResolverRegister;
use crate::public::AdapterOutgoingResponseResolverResolve;
use crate::public::AdapterOutgoingResponseTransformerLoadConfig;
use crate::public::AdapterOutgoingResponseTransformerRegister;
use crate::public::AdapterOutgoingResponseTransformerTransform;
use napi::threadsafe_function::ThreadSafeCallContext;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::Env;
use napi::JsFunction;
use napi::JsNumber;
use napi::JsObject;
use napi::JsString;
use napi::JsUnknown;
use serde::Deserialize;
use serde::Serialize;

pub fn worker_callback(
  env: &Env,
  child_receiver: Receiver<(AdapterOutgoingRequest, Sender<AdapterOutgoingResponse>)>,
  callback: JsFunction,
) {
  let tsfn = env
    .create_threadsafe_function(
      &callback,
      0,
      |ctx: ThreadSafeCallContext<(u8, AdapterOutgoingRequest)>| {
        // Return value is serialized
        let value = ctx.env.to_js_value(&ctx.value);
        Ok(vec![value])
      },
    )
    .unwrap();

  let unsafe_env = env.raw() as usize;

  thread::spawn(move || {
    while let Ok((action, response)) = child_receiver.recv() {
      let (tx, rx) = channel::<PromiseResult<AdapterOutgoingResponse>>();

      let action_n: u8 = match action {
        AdapterOutgoingRequest::Ping(_) => 0,
        AdapterOutgoingRequest::ResolverRegister(_) => 1,
        AdapterOutgoingRequest::ResolverLoadConfig(_) => 2,
        AdapterOutgoingRequest::ResolverResolve(_) => 3,
        AdapterOutgoingRequest::TransformerRegister(_) => 4,
        AdapterOutgoingRequest::TransformerLoadConfig(_) => 5,
        AdapterOutgoingRequest::TransformerTransform(_) => 6,
      };

      tsfn.call_with_return_value(
        Ok((action_n, action)),
        ThreadsafeFunctionCallMode::Blocking,
        move |result: JsUnknown| {
          let env = unsafe { Env::from_raw(unsafe_env as _) };

          if !result.is_promise()? {
            tx.send(PromiseResult::Ok(map_return_value(&env, result)?))
              .unwrap();
            return Ok(());
          }

          let result: JsObject = result.try_into()?;
          let then: JsFunction = result.get_named_property("then")?;

          let cb = env.create_function_from_closure("callback", {
            let tx = tx.clone();

            move |ctx| {
              let v = ctx.get::<JsUnknown>(0)?;
              tx.send(PromiseResult::Ok(map_return_value(&env, v)?))
                .unwrap();
              ctx.env.get_undefined()
            }
          })?;

          let eb = env.create_function_from_closure("error_callback", move |ctx| {
            let Ok(arg) = ctx.get::<JsString>(0) else {
              tx.send(PromiseResult::Err(
                "Worker Failure, unable to get error argument".to_string(),
              ))
              .unwrap();
              return ctx.env.get_undefined();
            };

            let Ok(err) = ctx.env.from_js_value::<String, JsString>(arg) else {
              tx.send(PromiseResult::Err(
                "Worker Failure, unable to cast type".to_string(),
              ))
              .unwrap();
              return ctx.env.get_undefined();
            };

            tx.send(PromiseResult::Err(err)).unwrap();
            ctx.env.get_undefined()
          })?;

          then.call(Some(&result), &[cb, eb])?;

          Ok(())
        },
      );

      match rx.recv().unwrap() {
        PromiseResult::Ok(value) => response.send(value).unwrap(),
        PromiseResult::Err(msg) => response.send(AdapterOutgoingResponse::Err(msg)).unwrap(),
      };
    }
  });
}

fn map_return_value(
  env: &Env,
  v: JsUnknown,
) -> napi::Result<AdapterOutgoingResponse> {
  let resp: JsObject = v.try_into()?;
  let key: JsNumber = resp.get("0")?.unwrap();
  let value: JsObject = resp.get("1")?.unwrap();
  let key = env.from_js_value::<u8, JsNumber>(key)?;

  Ok(match key {
    0 => AdapterOutgoingResponse::Ping(
      env.from_js_value::<AdapterOutgoingResponsePing, JsObject>(value)?,
    ),
    1 => AdapterOutgoingResponse::ResolverRegister(
      env.from_js_value::<AdapterOutgoingResponseResolverRegister, JsObject>(value)?,
    ),
    2 => AdapterOutgoingResponse::ResolverLoadConfig(
      env.from_js_value::<AdapterOutgoingResponseResolverLoadConfig, JsObject>(value)?,
    ),
    3 => AdapterOutgoingResponse::ResolverResolve(
      env.from_js_value::<AdapterOutgoingResponseResolverResolve, JsObject>(value)?,
    ),
    4 => AdapterOutgoingResponse::TransformerRegister(
      env.from_js_value::<AdapterOutgoingResponseTransformerRegister, JsObject>(value)?,
    ),
    5 => AdapterOutgoingResponse::TransformerLoadConfig(
      env.from_js_value::<AdapterOutgoingResponseTransformerLoadConfig, JsObject>(value)?,
    ),
    6 => AdapterOutgoingResponse::TransformerTransform(
      env.from_js_value::<AdapterOutgoingResponseTransformerTransform, JsObject>(value)?,
    ),
    _ => panic!("Invalid"),
  })
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PromiseResult<T> {
  Ok(T),
  Err(String),
}