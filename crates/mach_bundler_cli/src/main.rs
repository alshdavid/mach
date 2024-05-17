use mach_bundler_core::mach::{Mach, VersionOptions};

fn main() {
  let mach = Mach::new();
  let version = mach.version(VersionOptions{});
  println!("{}", version.pretty);
}

// #![deny(unused_crate_dependencies)]

// mod build;
// mod dev;
// mod version;
// mod watch;

// use std::time::SystemTime;

// use clap::Parser;
// use clap::Subcommand;
// use build::BuildCommand;
// use dev::DevCommand;
// use version::VersionCommand;
// use watch::WatchCommand;

// /*
//   Main just acts as a router to run CLI commands
// */
// #[derive(Parser, Debug)]
// struct Commands {
//   #[clap(subcommand)]
//   command: CommandType,
// }

// #[derive(Debug, Subcommand)]
// pub enum CommandType {
//   /// Build a project
//   Build(BuildCommand),
//   /// Start a web server and reload when changes are detected
//   Dev(DevCommand),
//   /// Build and rebuild when changes are detected
//   Watch(WatchCommand),
//   /// Print version information
//   Version(VersionCommand),
// }

// fn main() {
//   let start_time = SystemTime::now();

//   {
//     let command = Commands::parse();

//     match command.command {
//       CommandType::Build(command) => {
//         if let Err(msg) = build::main(command) {
//           println!("Build Error\n{}", msg);
//         };
//       }
//       CommandType::Dev(command) => {
//         dev::main(command);
//       }
//       CommandType::Watch(command) => {
//         watch::main(command);
//       }
//       CommandType::Version(_) => {
//         version::main();
//       }
//     }
//   }

//   println!(
//     "Total Time:      {:.3}s",
//     start_time.elapsed().unwrap().as_nanos() as f64 / 1_000_000 as f64 / 1000 as f64
//   );
// }
