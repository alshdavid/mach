use mach_libdeno::DenoConfig;
use mach_libdeno::DenoWrapper;
use std::path::Path;
use std::rc::Rc;

fn main() {
  let exe_path = std::env::current_exe().unwrap();
  let bin_path = exe_path.parent().unwrap();
  let lib_path = bin_path.join("libdeno.so");

  let mut deno = unsafe {
    let Ok(lib) = libloading::Library::new(lib_path) else {
      panic!();
    };
    let init_deno: libloading::Symbol<extern fn(Box<DenoConfig>) -> Box<DenoWrapper>> =
      lib.get(b"init_deno").unwrap();

    init_deno(Box::new(DenoConfig { threads: 1 }))
  };

  deno.ping_all();

  let f = Rc::new(());
  f.
}

// #![deny(unused_crate_dependencies)]

// mod cmd;
// mod kit;
// mod platform;
// mod public;

// use std::time::SystemTime;

// use clap::Parser;
// use clap::Subcommand;
// use cmd::build::BuildCommand;
// use cmd::dev::DevCommand;
// use cmd::version::VersionCommand;
// use cmd::watch::WatchCommand;

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
//         if let Err(msg) = cmd::build::main(command) {
//           println!("Build Error\n{}", msg);
//         };
//       }
//       CommandType::Dev(command) => {
//         cmd::dev::main(command);
//       }
//       CommandType::Watch(command) => {
//         cmd::watch::main(command);
//       }
//       CommandType::Version(_) => {
//         cmd::version::main();
//       }
//     }
//   }

//   println!(
//     "Total Time:    {:.3}s",
//     start_time.elapsed().unwrap().as_nanos() as f64 / 1_000_000 as f64 / 1000 as f64
//   );
// }
