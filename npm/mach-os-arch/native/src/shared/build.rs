use std::sync::Arc;

use mach_bundler_core::adapters::nodejs_napi::NodejsNapiAdapter;
use mach_bundler_core::public::AdapterMap;
use mach_bundler_core::BuildOptions;
use mach_bundler_core::BuildResult;
use mach_bundler_core::Mach;

use crate::shared::REGISTER_WORKER;
use crate::shared::START_WORKER;

pub fn mach_build_command(mut options: BuildOptions) -> Result<BuildResult, String> {
  let mach = Mach::new();

  let mut adapter_map = AdapterMap::new();

  // Setup Nodejs Plugin Runtime
  let tx_start_worker = START_WORKER.0.lock().unwrap().take().unwrap();
  let rx_register_worker = REGISTER_WORKER.1.lock().unwrap().take().unwrap();
  let worker_threads = options.node_workers.unwrap_or(num_cpus::get_physical()) as u8;

  let nodejs_adapter = NodejsNapiAdapter::new(tx_start_worker, rx_register_worker, worker_threads);
  adapter_map.insert("node".to_string(), Arc::new(nodejs_adapter));

  options.adapter_map = Some(adapter_map);

  mach.build(options)
}

// BuildOptions {
//   entries: command.entries,
//   out_folder: command.out_folder,
//   clean: command.clean,
//   optimize: !command.optimize,
//   bundle_splitting: command.bundle_splitting,
//   threads: command.threads,
//   node_workers: command.node_workers,
//   project_root: None,
//   adapter_map: Some(adapter_map),
// }
