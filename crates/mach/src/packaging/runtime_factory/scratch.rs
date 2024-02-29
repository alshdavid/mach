/*

mod adapters;
mod args;
mod bundling;
mod config;
mod packaging;
mod platform;
mod plugins;
mod public;
mod transformation;
mod emit;

use std::sync::Arc;

use swc_core::common::SourceMap;

use crate::{packaging::runtime_factory, platform::swc::render_stmts};

fn main() {
  let rf = runtime_factory::RuntimeFactory::new(Arc::new(SourceMap::default()));

  let stmt = rf.mach_require("module", &["test"]);

  let rendered = render_stmts(&vec![stmt], Arc::new(SourceMap::default()));

  println!("{}", rendered);
}


*/