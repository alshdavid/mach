use mach_bundler_core::adapters::nodejs_ipc::worker_ipc;
use napi::Env;
use napi::JsFunction;
use napi::JsUndefined;
use napi_derive::napi;

#[napi]
pub fn worker(
  env: Env,
  child_sender: String,
  child_receiver: String,
  callback: JsFunction,
) -> napi::Result<JsUndefined> {
  worker_ipc(
    env,
    child_sender,
    child_receiver,
    callback,
  )
}