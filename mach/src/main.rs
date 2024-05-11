// #![deny(unused_crate_dependencies)]

mod cmd;
mod kit;
mod platform;
mod public;

use std::io::BufReader;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use ipc_channel::ipc::IpcOneShotServer;
use ipc_channel::ipc::IpcReceiver;
use ipc_channel::ipc::IpcSender;
use ipc_channel::ipc::{self};
use kit::profiler::PROFILER;
use platform::adapters::nodejs::NodejsAdapter;
use platform::adapters::nodejs::NodejsAdapterOptions;
use platform::adapters::nodejs::NodejsManager;
use platform::adapters::nodejs::NodejsManagerOptions;
use platform::adapters::nodejs::NodejsWorker;
use public::nodejs::NodejsClientRequest;
use public::nodejs::NodejsHostRequest;
use serde::Serialize;
use tokio::task::JoinSet;

use crate::public::nodejs::NodejsHostResponse;

async fn main_async() {
  let nodejs_worker = NodejsAdapter::new(NodejsAdapterOptions {
    workers: 6,
  }).await;

  PROFILER.start("bench");
  let mut reqs = JoinSet::new();

  for _ in 0..100_000  {
    let nodejs_worker = nodejs_worker.clone();

    reqs.spawn(tokio::spawn(async move {
      nodejs_worker.ping_one().await
    }));
  }

  while let Some(result) = reqs.join_next().await {
    result.unwrap().unwrap();
  }
  PROFILER.lap("bench");

  // 

  PROFILER.log_millis_total("bench");
  thread::sleep(Duration::from_secs(2));
}

fn main() {
  tokio::runtime::Builder::new_multi_thread()
    .worker_threads(num_cpus::get_physical())
    .enable_all()
    .build()
    .unwrap()
    .block_on(main_async())
}


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
