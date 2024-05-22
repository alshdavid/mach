use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use divan::bench;
use divan::Bencher;
use once_cell::sync::Lazy;

use super::build_asset_graph;
use crate::platform::config::load_plugins;
use crate::platform::config::PluginContainerSync;
use crate::platform::config::ROOT_ASSET;
use crate::public::AdapterMap;
use crate::public::Compilation;
use crate::public::Dependency;
use crate::public::MachConfig;
use crate::public::MachConfigSync;
use crate::public::Machrc;

static CARGO_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
static FIXTURES: Lazy<PathBuf> = Lazy::new(|| {
  CARGO_DIR
    .join("src")
    .join("platform")
    .join("transformation")
    .join("__fixtures")
});

fn setup_root<T: AsRef<str>>(
  project_root: &Path,
  entries: &[T],
) -> (MachConfigSync, PluginContainerSync, Compilation) {
  let mach_config = Arc::new(MachConfig {
    entries: entries
      .iter()
      .map(|e| e.as_ref().to_string())
      .collect::<Vec<String>>(),
    project_root: project_root.to_owned(),
    ..Default::default()
  });

  let plugins = load_plugins(&mach_config, &Machrc::default(), &AdapterMap::new()).unwrap();
  let compilation = Compilation::new();
  (mach_config, plugins, compilation)
}

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

#[test]
fn should_produce_basic_asset_graph() {
  let (mach_config, plugins, mut c) = setup_root(&FIXTURES.join("js-esm-a"), &["./index.js"]);

  if let Err(msg) = build_asset_graph(mach_config, plugins, &mut c) {
    println!("{msg}");
    panic!()
  };

  c.asset_graph.traverse_from_asset(&ROOT_ASSET.id);

  // println!(
  //   "{:#?}",
  //   c.asset_graph
  //     .get_dependencies_for(&ROOT_ASSET.id)
  //     .collect::<Vec<&Dependency>>()
  // );
}
