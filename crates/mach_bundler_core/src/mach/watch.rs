#![allow(unused)]

use super::Mach;

#[derive(Debug)]
pub struct WatchOptions {}

pub struct WatchResult {}

impl Mach {
  pub fn watch(
    &self,
    options: WatchOptions,
  ) -> Result<WatchResult, String> {
    return Err("Not implemented yet".to_string());
  }
}
