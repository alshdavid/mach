mod napi_utils;

use ipc_channel_adapter::child::asynch::HostReceiver;
use ipc_channel_adapter::child::asynch::HostSender;
use napi::Env;
use napi::JsFunction;
use napi::JsUndefined;
use napi::JsUnknown;
use napi_derive::napi;
use napi_utils::await_promise;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::threadsafe_function::ThreadSafeCallContext;
use mach::public::nodejs::NodejsClientRequest;
use mach::public::nodejs::NodejsClientResponse;

use tokio::sync::mpsc::unbounded_channel;

use mach::public::nodejs::NodejsHostRequest;
use mach::public::nodejs::NodejsHostResponse;

#[napi]
pub fn run(env: Env, callback: JsFunction) -> napi::Result<JsUndefined> {
  let server_name = std::env::var("MACH_IPC_CHANNEL_1").unwrap().to_string();
  let (_, mut rx_ipc) = HostReceiver::<NodejsClientRequest, NodejsClientResponse>::new(&server_name).unwrap();

  let server_name = std::env::var("MACH_IPC_CHANNEL_2").unwrap().to_string();
  let _tx_ipc = HostSender::<NodejsHostRequest, NodejsHostResponse>::new(&server_name).unwrap();

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
    // let rx = HOST_RECEIVER.subscribe();
  
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
