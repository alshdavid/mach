use std::path::PathBuf;
use std::sync::Arc;

use divan::bench;
use divan::Bencher;
use once_cell::sync::Lazy;

use super::super::build_asset_graph;
use super::utils::CARGO_DIR;
use crate::core::plugins::load_plugins;
use crate::public::Compilation;
use crate::public::MachConfig;
use crate::public::Machrc;
use crate::rpc::RpcHosts;

pub static BENCHMARK_FIXTURE: Lazy<Option<PathBuf>> = Lazy::new(|| {
  let target = CARGO_DIR
    .parent()
    .unwrap()
    .parent()
    .unwrap()
    .join("benchmarks")
    .join("mach_1");
  if target.exists() {
    Some(target)
  } else {
    None
  }
});

#[bench]
fn bench_build_asset_graph(b: Bencher) {
  let Some(project_root) = &*BENCHMARK_FIXTURE else {
    println!("Missing benchmark fixture");
    return;
  };

  let mach_config = Arc::new(MachConfig {
    entries: vec!["./src/index.js".to_string()],
    project_root: project_root.clone(),
    ..Default::default()
  });

  let plugins = load_plugins(&mach_config, &Machrc::default(), &RpcHosts::new()).unwrap();
  let compilation = Compilation::new();

  b.bench_local(move || {
    build_asset_graph(
      mach_config.clone(),
      plugins.clone(),
      &mut compilation.clone(),
    )
  });
}
