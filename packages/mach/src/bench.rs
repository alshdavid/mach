#![allow(unused)]

pub mod cmd;
pub mod kit;
pub mod core;
pub mod plugins;
pub mod public;
pub mod rpc;

fn main() {
  divan::main();
}
