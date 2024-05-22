use std::path::PathBuf;

use divan::bench;
use divan::Bencher;

use crate::public::MachConfig;

#[bench]
fn build_asset_graph(b: Bencher) {
  let mach_config = MachConfig {
    entries: vec![],
    project_root: PathBuf::from("/Users/dalsh/Development/alshdavid/mach/benchmarks/mach_100"),
    ..Default::default()
  };
  b.bench_local(move || {});
}
