#![deny(unused_crate_dependencies)]
use std::time::SystemTime;

use mach_bundler_core::cli::parse_options_from_cli;
use mach_bundler_core::cli::CommandType;
use mach_bundler_core::BuildOptions;
use mach_bundler_core::DevOptions;
use mach_bundler_core::Mach;
use mach_bundler_core::VersionOptions;
use mach_bundler_core::WatchOptions;

fn main() {
  let command = parse_options_from_cli();
  let mach = Mach::new();

  match command.command {
    CommandType::Build(command) => {
      let start_time = SystemTime::now();

      if let Err(msg) = mach.build(BuildOptions {
        entries: command.entries,
        out_folder: command.out_folder,
        clean: command.clean,
        optimize: !command.no_optimize,
        bundle_splitting: command.bundle_splitting,
        threads: command.threads,
        node_workers: command.node_workers,
        project_root: command.project_root,
        adapter_map: None,
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

  
}

// fn time_elapsed(time: &SystemTime) -> f64 {
//   time.elapsed().unwrap().as_nanos() as f64 / 1_000_000 as f64 / 1000 as f64
// }

// fn report_timings(
//   start_time: SystemTime,
//   rx: Receiver<BuildEvent>
// ) -> impl Fn() {
//   move || {
//     let mut init_time = None::<SystemTime>;
//     let mut transform_time = None::<SystemTime>;
//     let mut bundling_time = None::<SystemTime>;
//     let mut packaging_time = None::<SystemTime>;
//     let mut emit_time = None::<SystemTime>;
    
//     while let Ok(build_event) = rx.recv() {
//       if let BuildEvent::InitializationComplete{ timestamp } = build_event {
//         println!("Init: {:.3}", time_elapsed(&start_time));
//         init_time.replace(timestamp);
//       }
//       if let BuildEvent::TransformationComplete{ timestamp } = build_event {
//         println!("Transform: {:.3}", time_elapsed(&init_time.unwrap()));
//         transform_time.replace(timestamp);
//       }
//       if let BuildEvent::BundlingComplete{ timestamp } = build_event {
//         println!("Bundling: {:.3}", time_elapsed(&transform_time.unwrap()));
//         bundling_time.replace(timestamp);
//       }
//       if let BuildEvent::PackagingComplete{ timestamp } = build_event {
//         println!("Packaging: {:.3}", time_elapsed(&bundling_time.unwrap()));
//         packaging_time.replace(timestamp);
//       }
//       if let BuildEvent::BuildComplete{ timestamp } = build_event {
//         println!("Emit: {:.3}", time_elapsed(&packaging_time.unwrap()));
//         emit_time.replace(timestamp);
//       }
//     }
//   }
// }