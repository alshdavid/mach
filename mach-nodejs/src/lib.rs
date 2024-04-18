mod ipc;

use ipc::HostReceiver;
use ipc::HostSender;
use mach::public::nodejs::NodejsClientRequest;
use mach::public::nodejs::NodejsClientResponse;
use napi::Env;
use napi::JsFunction;
use napi::JsUndefined;
use napi_derive::napi;
use once_cell::sync::Lazy;

static HOST_SENDER: Lazy<HostSender> = Lazy::new(|| HostSender::new());
static HOST_RECEIVER: Lazy<HostReceiver> = Lazy::new(|| HostReceiver::new());

#[napi]
pub fn on_ping(env: Env, callback: JsFunction) -> napi::Result<JsUndefined> {
  let rx = HOST_RECEIVER.subscribe();

  while let Ok((req, res)) = rx.recv() {
    let NodejsClientRequest::Ping = req else {
      continue;
    };
    callback.call_without_args(None).unwrap();
    res.send(NodejsClientResponse::Ping).unwrap();
  }

  env.get_undefined()
}

#[napi]
pub fn on_resolver_register(env: Env, callback: JsFunction) -> napi::Result<JsUndefined> {
  println!("registed");
  let rx = HOST_RECEIVER.subscribe();

  while let Ok((req, res)) = rx.recv() {
    let NodejsClientRequest::ResolverRegister(req) = req else {
      continue;
    };
    let pass = env.create_string(&req).unwrap();
    callback.call(None, &[pass]).unwrap();
    res.send(NodejsClientResponse::Ping).unwrap();
  }

  env.get_undefined()
}
