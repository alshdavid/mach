use std::fmt::Debug;
use std::sync::mpsc::Sender;

use napi::*;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub enum PromiseResult<T> {
  Ok(T),
  Err(String),
}

pub fn await_promise<T>(
  env: Env,
  result: JsUnknown,
  tx: Sender<PromiseResult<T>>,
) -> napi::Result<()>
where
  T: DeserializeOwned + Send + Debug + 'static,
{
  if !result.is_promise()? {
    let res: T = env.from_js_value(result)?;
    tx.send(PromiseResult::Ok(res)).unwrap();
    return Ok(());
  }

  let result: JsObject = result.try_into()?;
  let then: JsFunction = result.get_named_property("then")?;

  let cb = env.create_function_from_closure("callback", {
    let tx = tx.clone();

    move |ctx| {
      let v = ctx.get::<JsUnknown>(0)?;
      let res = ctx.env.from_js_value::<T, JsUnknown>(v).expect("msg");
      tx.send(PromiseResult::Ok(res)).expect("send failure");
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
}
