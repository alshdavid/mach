mod ipc;
mod napi_utils;

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

use ipc::HostReceiver;
use ipc::HostSender;
use mach::public::nodejs::NodejsClientRequest;
use mach::public::nodejs::NodejsClientResponse;
use napi::Env;
use napi::JsFunction;
use napi::JsObject;
use napi::JsUndefined;
use napi::JsUnknown;
use napi_derive::napi;
use napi_utils::await_promise;
use once_cell::sync::Lazy;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedSender;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::threadsafe_function::ThreadSafeCallContext;

use crate::napi_utils::create_async_callback;

static HOST_SENDER: Lazy<HostSender> = Lazy::new(|| HostSender::new());
static HOST_RECEIVER: Lazy<HostReceiver> = Lazy::new(|| HostReceiver::new());

#[napi]
pub fn run(env: Env, callback: JsFunction) -> napi::Result<JsUndefined> {
  let tsfn = env.create_threadsafe_function(
    &callback, 
    0,
    |ctx: ThreadSafeCallContext<NodejsClientRequest>| {
      // Return value is serialized
      let value = ctx.env.to_js_value(&ctx.value);
      Ok(vec![value])
    },
  ).unwrap();

  let unsafe_env = env.raw() as usize;

  env.spawn_future(async move {
    let rx = HOST_RECEIVER.subscribe();
  
    while let Ok((action, response)) = rx.recv() {
      let (tx, mut rx) = unbounded_channel::<NodejsClientResponse>();

      tsfn.call_with_return_value(
        Ok(action),
        ThreadsafeFunctionCallMode::Blocking,
        move |result: JsUnknown| {
          let env = unsafe { Env::from_raw(unsafe_env as _) };
          await_promise(env, result, tx)?;
          Ok(())
        },
      );

      response.send(rx.recv().await.unwrap()).unwrap();
    }
    Ok(())
  })?;

  // while let Ok((action, response)) = rx.recv() {
  //   match action {
  //     NodejsClientRequest::Ping => {
  //       cb.call(None, &[
  //         env.create_int32(0)?.into_unknown(),
  //         env.get_undefined()?.into_unknown(),
  //         env.create_function_from_closure("cb", move |_| {
  //           response.send(NodejsClientResponse::Ping).unwrap();
  //           Ok(())
  //         })?.into_unknown(),
  //       ])?;
  //     }
  //     NodejsClientRequest::ResolverRegister(specifier) => {
  //       cb.call(None, &[
  //         env.create_int32(1)?.into_unknown(),
  //         env.create_string(&specifier)?.into_unknown(),
  //         env.create_function_from_closure("cb", move |_| {
  //           response.send(NodejsClientResponse::ResolverRegister).unwrap();
  //           Ok(())
  //         })?.into_unknown(),
  //       ])?;
  //     }
  //   }
  // }

  env.get_undefined()
}
