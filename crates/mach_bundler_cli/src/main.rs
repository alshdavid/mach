#![deny(unused_crate_dependencies)]
use std::time::SystemTime;

use mach_bundler_core::cli::parse_options_from_cli;
use mach_bundler_core::cli::CommandType;
use mach_bundler_core::Mach;
use mach_bundler_core::BuildOptions;
use mach_bundler_core::DevOptions;
use mach_bundler_core::WatchOptions;
use mach_bundler_core::VersionOptions;

fn main() {
  let command = parse_options_from_cli();
  let mach = Mach::new();

  let start_time = SystemTime::now();

  match command.command {
    CommandType::Build(command) => {
      if let Err(msg) = mach.build(BuildOptions {
        entries: command.entries,
        out_folder: command.out_folder,
        clean: command.clean,
        optimize: !command.no_optimize,
        bundle_splitting: command.bundle_splitting,
        threads: command.threads,
        node_workers: command.node_workers,
        debug: command.debug,
    }) {
        println!("Build Error\n{}", msg);
      };
    }
    CommandType::Dev(_command) => {
      if let Err(msg) = mach.dev(DevOptions {}) {
        println!("Dev Error\n{}", msg);
      };
    }
    CommandType::Watch(_command) => {
      if let Err(msg) = mach.watch(WatchOptions {}) {
        println!("Watch Error\n{}", msg);
      };
    }
    CommandType::Version(_command) => {
      let result = mach.version(VersionOptions {});
      println!("{}", result.pretty);
    }
  }

  println!(
    "Total Time:      {:.3}s",
    start_time.elapsed().unwrap().as_nanos() as f64 / 1_000_000 as f64 / 1000 as f64
  );
}
