#![allow(unused)]

#[cfg(feature = "cli_parser")]
use clap::Parser;

use super::Mach;

#[cfg_attr(feature = "cli_parser", derive(Parser))]
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
