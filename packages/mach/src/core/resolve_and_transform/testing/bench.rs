// use std::path::PathBuf;
// use std::sync::Arc;

// use divan::bench;
// use divan::Bencher;

// use super::super::build_asset_graph;
// use crate::core::plugins::load_plugins;
// use crate::public::Compilation;
// use crate::public::MachConfig;
// use crate::public::Machrc;
// use crate::rpc::RpcHosts;

// #[bench]
// fn bench_build_asset_graph(b: Bencher) {
//   let project_root = PathBuf::from("/Users/dalsh/Development/alshdavid/mach/benchmarks/mach_100");

//   let mach_config = Arc::new(MachConfig {
//     entries: vec!["./src/index.js".to_string()],
//     project_root: project_root.clone(),
//     ..Default::default()
//   });

//   let plugins = load_plugins(&mach_config, &Machrc::default(), &RpcHosts::new()).unwrap();
//   let compilation = Compilation::new();

//   b.bench_local(move || {
//     build_asset_graph(
//       mach_config.clone(),
//       plugins.clone(),
//       &mut compilation.clone(),
//     )
//   });
// }
