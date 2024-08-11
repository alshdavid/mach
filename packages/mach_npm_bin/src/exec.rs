/*
  This contains the bindings for the Nodejs CLI usage of Mach
*/
use std::thread;
use std::time::SystemTime;

use mach_bundler_core::cli::MachCommand;
use mach_bundler_core::cli::MachCommandType;
use mach_bundler_core::DevOptions;
use mach_bundler_core::Mach;
use mach_bundler_core::MachOptions;
use mach_bundler_core::VersionOptions;
use mach_bundler_core::WatchOptions;
use napi_derive::napi;

#[napi]
pub fn exec(args: Vec<String>) -> napi::Result<()> {
  thread::spawn(move || {
    let start_time = SystemTime::now();

    let command = match MachCommand::from_args(&args) {
      Ok(command) => command,
      Err(error) => {
        eprintln!("{}", error);
        return Err(());
      },
    };

    let mach = Mach::new(MachOptions::default());

    match command.command {
      MachCommandType::Build(_command) => {
        println!(
          "🚀 Build Success ({:.3}s)",
          start_time.elapsed().unwrap().as_nanos() as f64 / 1_000_000 as f64 / 1000 as f64
        );
      }
      MachCommandType::Dev(_command) => {
        if let Err(msg) = mach.dev(DevOptions {}) {
          println!("❌ Dev Error\n{}", msg);
        };
      }
      MachCommandType::Watch(_command) => {
        if let Err(msg) = mach.watch(WatchOptions {}) {
          println!("❌ Watch Error\n{}", msg);
        };
      }
      MachCommandType::Version(_command) => {
        let result = mach.version(VersionOptions {});
        println!("{}", result.pretty);
      }
    }

    Ok(())
  });

  return Ok(());
}
