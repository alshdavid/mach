use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

use mach_bundler_core::adapters::nodejs_napi::NodejsNapiAdapter;
use mach_bundler_core::cli::parse_options;
use mach_bundler_core::cli::CommandType;
use mach_bundler_core::public::AdapterMap;
use mach_bundler_core::BuildOptions;
use mach_bundler_core::DevOptions;
use mach_bundler_core::Mach;
use mach_bundler_core::VersionOptions;
use mach_bundler_core::WatchOptions;
use napi_derive::napi;

use crate::shared::REGISTER_WORKER;
use crate::shared::START_WORKER;

#[napi]
pub fn exec(args: Vec<String>) {
  thread::spawn(move || {
    let start_time = SystemTime::now();
    let command = parse_options(&args).unwrap();
    let mach = Mach::new();

    match command.command {
      CommandType::Build(command) => {
        let mut adapter_map = AdapterMap::new();

        // Setup Nodejs Plugin Runtime
        let tx_start_worker = START_WORKER.0.lock().unwrap().take().unwrap();
        let rx_register_worker = REGISTER_WORKER.1.lock().unwrap().take().unwrap();
        let worker_threads = command.node_workers.unwrap_or(num_cpus::get_physical()) as u8;

        let nodejs_adapter =
          NodejsNapiAdapter::new(tx_start_worker, rx_register_worker, worker_threads);
        adapter_map.insert("node".to_string(), Arc::new(nodejs_adapter));

        if let Err(msg) = mach.build(BuildOptions {
          entries: command.entries,
          out_folder: command.out_folder,
          clean: command.clean,
          optimize: !command.no_optimize,
          bundle_splitting: command.bundle_splitting,
          threads: command.threads,
          node_workers: command.node_workers,
          project_root: None,
          adapter_map: Some(adapter_map),
        }) {
          println!("❌ Build Failure\n{}", msg);
          return;
        };

        println!(
          "🚀 Build Success ({:.3}s)",
          start_time.elapsed().unwrap().as_nanos() as f64 / 1_000_000 as f64 / 1000 as f64
        );
      }
      CommandType::Dev(_command) => {
        if let Err(msg) = mach.dev(DevOptions {}) {
          println!("❌ Dev Error\n{}", msg);
        };
      }
      CommandType::Watch(_command) => {
        if let Err(msg) = mach.watch(WatchOptions {}) {
          println!("❌ Watch Error\n{}", msg);
        };
      }
      CommandType::Version(_command) => {
        let result = mach.version(VersionOptions {});
        println!("{}", result.pretty);
      }
    }
  });
}
