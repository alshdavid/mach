#![allow(unused)]

pub mod rpc;
#[cfg(feature = "cli_parser")]
pub mod cli;
pub mod kit;
pub mod mach;
pub mod platform;
pub mod plugins;
pub mod public;

pub use self::mach::*;

fn main() {
  divan::main();
}
