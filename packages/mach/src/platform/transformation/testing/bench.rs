use std::path::PathBuf;
use std::sync::Arc;

use divan::bench;
use divan::Bencher;

use super::super::build_asset_graph;
use crate::platform::config::load_plugins;
use crate::public::AdapterMap;
use crate::public::Compilation;
use crate::public::MachConfig;
use crate::public::Machrc;

#[bench]
fn bench_build_asset_graph(b: Bencher) {
  let project_root = PathBuf::from("/Users/dalsh/Development/alshdavid/mach/benchmarks/mach_100");

  let mach_config = Arc::new(MachConfig {
    entries: vec!["./src/index.js".to_string()],
    project_root: project_root.clone(),
    ..Default::default()
  });

  let plugins = load_plugins(&mach_config, &Machrc::default(), &AdapterMap::new()).unwrap();
  let compilation = Compilation::new();

  b.bench_local(move || {
    build_asset_graph(
      mach_config.clone(),
      plugins.clone(),
      &mut compilation.clone(),
    )
  });
}