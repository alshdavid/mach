use napi::Env;
use napi::JsFunction;
use napi::JsObject;
use napi::JsUndefined;
use napi_derive::napi;

use super::build::build;

#[napi]
pub struct Mach {}

#[napi]
impl Mach {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {}
  }

  #[napi]
  pub fn build(
    &self,
    env: Env,
    options: JsObject,
    callback: JsFunction,
  ) -> napi::Result<JsUndefined> {
    build(env, options, callback)
  }
}
