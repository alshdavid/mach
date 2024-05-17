use napi_derive::napi;

#[napi]
pub struct Mach {}

#[napi]
impl Mach {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
    }
  }
}