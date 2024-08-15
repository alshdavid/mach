#![allow(unused)]

pub mod cmd;
pub mod core;
pub mod kit;
pub mod plugins;
pub mod public;
pub mod rpc;

fn main() {
  divan::main();
}
