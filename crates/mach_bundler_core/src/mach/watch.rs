#![allow(unused)]

#[cfg(feature = "cli_parser")]
use clap::Parser;

use super::Mach;

#[cfg_attr(feature = "cli_parser", derive(Parser))]
#[derive(Debug)]
pub struct WatchOptions {}

pub struct WatchResult {

}

impl Mach {
  pub fn watch(&self, options: WatchOptions) -> Result<WatchResult, String> {
    return Err("Not implemented yet".to_string());
  }
}
