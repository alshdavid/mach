use std::path::PathBuf;
use std::sync::Arc;

use divan::bench;
use divan::Bencher;

use super::resolve_and_transform;
use crate::platform::config::load_plugins;
use crate::public::AdapterMap;
use crate::public::Compilation;
use crate::public::MachConfig;
use crate::public::Machrc;

#[bench]
fn build_asset_graph(b: Bencher) {
  let project_root = PathBuf::from("/Users/dalsh/Development/alshdavid/mach/benchmarks/mach_100");

  let mach_config = Arc::new(MachConfig {
    entries: vec!["./src/index.js".to_string()],
    project_root: project_root.clone(),
    ..Default::default()
  });

  let plugins = load_plugins(&mach_config, &Machrc::default(), &AdapterMap::new()).unwrap();
  let compilation = Compilation::new();

  b.bench_local(move || {
    resolve_and_transform(mach_config.clone(), plugins.clone(), compilation.clone())
  });
}
