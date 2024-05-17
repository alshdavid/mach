use napi::Env;
use napi::JsObject;
use napi::JsUnknown;
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
  ) -> napi::Result<JsUnknown> {
    build(env, options)
  }
}
