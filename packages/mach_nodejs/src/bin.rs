use std::thread;
use std::time::SystemTime;

use mach_bundler_core::core::cli::parse_options;
use mach_bundler_core::core::cli::CommandType;
// use mach_bundler_core::BuildOptions;
use mach_bundler_core::DevOptions;
use mach_bundler_core::Mach;
use mach_bundler_core::MachOptions;
use mach_bundler_core::VersionOptions;
use mach_bundler_core::WatchOptions;
use napi_derive::napi;

// use crate::cmd;

// use crate::shared::mach_build_command;

#[napi]
pub fn exec(args: Vec<String>) {
  thread::spawn(move || {
    let start_time = SystemTime::now();
    let command = parse_options(&args).unwrap();
    let mach = Mach::new(MachOptions::default());

    match command.command {
      CommandType::Build(_command) => {
        // if let Err(msg) = cmd::build(BuildOptions {
        //   entries: command.entries,
        //   out_folder: command.out_folder,
        //   clean: command.clean,
        //   optimize: !command.no_optimize,
        //   bundle_splitting: command.bundle_splitting,
        //   threads: command.threads,
        //   node_workers: command.node_workers,
        //   project_root: command.project_root,
        //   adapter_map: None,
        // }) {
        //   println!("❌ Build Failure\n{}", msg);
        //   return;
        // };

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
