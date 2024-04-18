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
use kit::broadcast_channel::channel_broadcast;
use kit::ipc::sync::IpcHost;
use kit::profiler::PROFILER;
use platform::adapters::nodejs::NodejsManager;
use platform::adapters::nodejs::NodejsManagerOptions;
use platform::adapters::nodejs::NodejsWorker;
use public::nodejs::NodejsClientRequest;
use public::nodejs::NodejsHostRequest;
use serde::Serialize;

use crate::public::nodejs::NodejsHostResponse;

fn main() {
  let nodejs_worker = NodejsManager::new(NodejsManagerOptions {
    workers: 3,
  });

  let rx1 = nodejs_worker.send(NodejsClientRequest::Ping);

  let rx2 = nodejs_worker.on.subscribe();
  while let Ok((req, res)) = rx2.recv() {
    println!("From child {:?}", req);
    res.send(NodejsHostResponse::Ping).unwrap();
  }

  rx1.recv().unwrap();
}


/*
  let nodejs_worker = NodejsManager::new(NodejsManagerOptions {
    workers: std::env::var("NODEJS_WORKERS").unwrap_or("1".to_string()).parse().unwrap_or(1),
  });

  let n = 6;
  let mut v = vec![];

  PROFILER.start(&format!("ping"));
  for t in 0..n {
    let rx = nodejs_worker.send_ping();
    v.push(rx);
  }

  for rx in v {
    rx.recv().unwrap();
  }
  PROFILER.lap(&format!("ping"));
  PROFILER.log_millis_total(&format!("ping"));

  thread::sleep(Duration::from_secs(2));



 // let w = 1;
  // // let n = 1;
  // let n = 100_000;
  // let t = 3;
  // let nw = n/w;

  // println!("{} / {} = {}", w, n, nw);

  // for t in 0..t {
  //   let nodejs = Nodejs::new(NodejsOptions {
  //     workers: w,
  //     nodejs_worker_factory: Arc::new(NodejsInstanceIpc::new()),
  //   });

  //   PROFILER.start(&format!("stdio {}", t));
  //   let mut v = vec![];

  //   for w in 0..w {
  //     let nodejs = nodejs.clone();

  //     v.push(thread::spawn(move || {
  //       let mut v2 = vec![];

  //       for i in 0..nw {
  //         // thread::sleep(Duration::from_nanos(1));
  //         // let mut v = serde_json::to_vec(&(0, None::<()>)).unwrap();
  //         let resp = nodejs.request(vec![]);
  //         v2.push(resp);
  //       }

  //       for v in v2 {
  //         let v = v.recv().unwrap();
  //         // println!("{:?}", v)
  //       }
  //     }));
  //   }

  //   for v in v {
  //     v.join().unwrap()
  //   }

  //   PROFILER.lap(&format!("stdio {}", t));
  //   PROFILER.log_millis_total(&format!("stdio {}", t));
  // }

*/
 // let w = 1;
  // // let n = 1;
  // let n = 100_000;
  // let t = 3;
  // let nw = n/w;

  // println!("{} / {} = {}", w, n, nw);

  // for t in 0..t {
  //   let nodejs = Nodejs::new(NodejsOptions {
  //     workers: w,
  //     nodejs_worker_factory: Arc::new(NodejsInstanceIpc::new()),
  //   });

  //   PROFILER.start(&format!("stdio {}", t));
  //   let mut v = vec![];

  //   for w in 0..w {
  //     let nodejs = nodejs.clone();

  //     v.push(thread::spawn(move || {
  //       let mut v2 = vec![];

  //       for i in 0..nw {
  //         // thread::sleep(Duration::from_nanos(1));
  //         // let mut v = serde_json::to_vec(&(0, None::<()>)).unwrap();
  //         let resp = nodejs.request(vec![]);
  //         v2.push(resp);
  //       }

  //       for v in v2 {
  //         let v = v.recv().unwrap();
  //         // println!("{:?}", v)
  //       }
  //     }));
  //   }

  //   for v in v {
  //     v.join().unwrap()
  //   }

  //   PROFILER.lap(&format!("stdio {}", t));
  //   PROFILER.log_millis_total(&format!("stdio {}", t));
  // }


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
