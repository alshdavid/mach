#![allow(unused)]

use super::Mach;

#[derive(Debug)]
pub struct DevOptions {}

pub struct DevResult {}

impl Mach {
  pub fn dev(
    &self,
    options: DevOptions,
  ) -> Result<DevResult, String> {
    return Err("Not implemented yet".to_string());
  }
}
