use mach_bundler_core::rpc::nodejs::RpcConnectionNodejs;
use napi::Env;
use napi::JsFunction;
use napi::JsUndefined;
use napi_derive::napi;

#[napi]
pub fn worker_callback(env: Env, callback: JsFunction) -> napi::Result<JsUndefined> {
  RpcConnectionNodejs::create_worker_callback(&env, callback)?;
  env.get_undefined()
}