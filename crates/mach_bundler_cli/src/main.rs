#![deny(unused_crate_dependencies)]
use std::time::SystemTime;

use mach_bundler_core::cli::parse_options_from_cli;
use mach_bundler_core::cli::CommandType;
use mach_bundler_core::Mach;

fn main() {
  let command = parse_options_from_cli();
  let mach = Mach::new();

  let start_time = SystemTime::now();

  match command.command {
    CommandType::Build(options) => {
      if let Err(msg) = mach.build(options) {
        println!("Build Error\n{}", msg);
      };
    }
    CommandType::Dev(options) => {
      if let Err(msg) = mach.dev(options) {
        println!("Dev Error\n{}", msg);
      };
    }
    CommandType::Watch(options) => {
      if let Err(msg) = mach.watch(options) {
        println!("Watch Error\n{}", msg);
      };
    }
    CommandType::Version(options) => {
      let result = mach.version(options);
      println!("{}", result.pretty);
    }
  }

  println!(
    "Total Time:      {:.3}s",
    start_time.elapsed().unwrap().as_nanos() as f64 / 1_000_000 as f64 / 1000 as f64
  );
}
