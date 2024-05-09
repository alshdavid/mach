mod ipc;

use ipc::HostReceiver;
use ipc::HostSender;
use mach::public::nodejs::NodejsClientRequest;
use mach::public::nodejs::NodejsClientResponse;
use napi::bindgen_prelude::Promise;
use napi::Env;
use napi::JsFunction;
use napi::JsObject;
use napi::JsUndefined;
use napi_derive::napi;
use once_cell::sync::Lazy;

static HOST_SENDER: Lazy<HostSender> = Lazy::new(|| HostSender::new());
static HOST_RECEIVER: Lazy<HostReceiver> = Lazy::new(|| HostReceiver::new());

#[napi]
pub fn run(env: Env, cb: JsFunction) -> napi::Result<JsUndefined> {
  let rx = HOST_RECEIVER.subscribe();

  while let Ok((action, response)) = rx.recv() {
    match action {
      NodejsClientRequest::Ping => {
        cb.call(None, &[
          env.create_int32(0)?.into_unknown(),
          env.get_undefined()?.into_unknown(),
          env.create_function_from_closure("cb", move |_| {
            response.send(NodejsClientResponse::Ping).unwrap();
            Ok(())
          })?.into_unknown(),
        ])?;
      }
      NodejsClientRequest::ResolverRegister(specifier) => {
        cb.call(None, &[
          env.create_int32(1)?.into_unknown(),
          env.create_string(&specifier)?.into_unknown(),
          env.create_function_from_closure("cb", move |_| {
            response.send(NodejsClientResponse::ResolverRegister).unwrap();
            Ok(())
          })?.into_unknown(),
        ])?;
      }
    }
  }

  env.get_undefined()
}
