#![allow(unused)]

#[cfg(feature = "cli_parser")]
pub mod cli;
pub mod cmd;
pub mod kit;
pub mod platform;
pub mod plugins;
pub mod public;
pub mod rpc;

fn main() {
  divan::main();
}
