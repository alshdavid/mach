mod napi_utils;

use ipc_channel_adapter::child::asynch::HostReceiver;
use ipc_channel_adapter::child::asynch::HostSender;
use napi::Env;
use napi::JsFunction;
use napi::JsUndefined;
use napi::JsUnknown;
use napi_derive::napi;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::threadsafe_function::ThreadSafeCallContext;
use mach::public::nodejs::NodejsClientRequest;
use mach::public::nodejs::NodejsClientResponse;

use napi_utils::await_promise;
use tokio::sync::mpsc::unbounded_channel;

use mach::public::nodejs::NodejsHostRequest;
use mach::public::nodejs::NodejsHostResponse;

#[napi]
pub fn run(
  env: Env, 
  child_sender: String,
  child_receiver: String,
  callback: JsFunction,
) -> napi::Result<JsUndefined> {
  let (_, mut rx_ipc) = HostReceiver::<NodejsClientRequest, NodejsClientResponse>::new(&child_sender).unwrap();
  let _tx_ipc = HostSender::<NodejsHostRequest, NodejsHostResponse>::new(&child_receiver).unwrap();

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
    while let Some((action, response)) = rx_ipc.recv().await {
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
  
  env.get_undefined()
}
