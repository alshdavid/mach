#![allow(unused)]

pub mod rpc;
#[cfg(feature = "cli_parser")]
pub mod cli;
pub mod cmd;
pub mod kit;
pub mod mach;
pub mod platform;
pub mod plugins;
pub mod public;

fn main() {
  divan::main();
}
