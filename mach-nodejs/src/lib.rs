mod napi_utils;

use std::sync::mpsc::channel;
use std::thread;

use ipc_channel_adapter::child::sync::HostReceiver;
use ipc_channel_adapter::child::sync::HostSender;
use mach::public::nodejs::client::NodejsClientRequest;
use mach::public::nodejs::client::NodejsClientResponse;
use mach::public::nodejs::NodejsHostRequest;
use mach::public::nodejs::NodejsHostResponse;
use napi::threadsafe_function::ThreadSafeCallContext;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::Env;
use napi::JsFunction;
use napi::JsUndefined;
use napi::JsUnknown;
use napi_derive::napi;
use napi_utils::await_promise;

#[napi]
pub fn run(
  env: Env,
  child_sender: String,
  child_receiver: String,
  callback: JsFunction,
) -> napi::Result<JsUndefined> {
  let (_, rx_ipc) =
    HostReceiver::<NodejsClientRequest, NodejsClientResponse>::new(&child_sender).unwrap();

  let _tx_ipc = HostSender::<NodejsHostRequest, NodejsHostResponse>::new(&child_receiver).unwrap();

  let tsfn = env
    .create_threadsafe_function(
      &callback,
      0,
      |ctx: ThreadSafeCallContext<NodejsClientRequest>| {
        // Return value is serialized
        let value = ctx.env.to_js_value(&ctx.value);
        Ok(vec![value])
      },
    )
    .unwrap();

  let unsafe_env = env.raw() as usize;

  thread::spawn(move || {
    while let Ok((action, response)) = rx_ipc.recv() {
      let (tx, rx) = channel::<NodejsClientResponse>();

      tsfn.call_with_return_value(
        Ok(action),
        ThreadsafeFunctionCallMode::Blocking,
        move |result: JsUnknown| {
          let env = unsafe { Env::from_raw(unsafe_env as _) };
          await_promise(env, result, tx)?;
          Ok(())
        },
      );

      let reply = rx.recv().unwrap();
      response.send(reply).unwrap();
    }
  });

  env.get_undefined()
}
