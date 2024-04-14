// #![deny(unused_crate_dependencies)]

mod cmd;
mod kit;
mod platform;
mod public;

use std::io::Write;
use std::thread;
use std::time::Duration;

use kit::profiler::PROFILER;
use platform::adapters::nodejs::{NodejsInstanceStdio, NodejsInstanceTcp};
// use platform::ipc::nodejs::NodejsWorkerFarm;
use serde::Serialize;

use crate::platform::adapters::nodejs::NodejsInstanceTcpOptions;

fn main() {
  // let nodejs_workers = NodejsWorkerFarm::new(2);

  // let nodejs_workers1 = nodejs_workers.clone();
  // thread::spawn(move || {
  //   let rx = nodejs_workers1.subscribe();
  //   while let Ok(bytes) = rx.recv() {
  //     println!("{:?}", bytes);
  //   }
  //   thread::sleep(Duration::from_secs(2));
  // });

  // let nodejs_workers2 = nodejs_workers.clone();
  // thread::spawn(move || {
  //   for _ in 0..6 {
  //     thread::sleep(Duration::from_millis(5));
  //     nodejs_workers2.send(vec![0, 0, 10]);
  //   }
  //   thread::sleep(Duration::from_secs(2));
  // });

  // thread::sleep(Duration::from_secs(2));

  // let nodejs_workers3 = nodejs_workers.clone();
  // thread::spawn(move || {
  //   let mut b = Vec::<u8>::from(&[1, 0]);
  //   b.extend(serde_json::to_vec("ping").unwrap());
  //   b.push(10);

  //   for _ in 0..10 {
  //     // println!("[2]");
  //     thread::sleep(Duration::from_millis(5));
  //     let i = nodejs_workers3.send(&b);
  //     println!("[2] {}", i);
  //   }
  // });

  // let nodejs_workers4 = nodejs_workers.clone();
  // thread::spawn(move || {
  //   let mut b = Vec::<u8>::from(&[1, 0]);
  //   b.extend(serde_json::to_vec("ping").unwrap());
  //   b.push(10);

  //   for _ in 0..10 {
  //     // println!("[3]");
  //     thread::sleep(Duration::from_millis(5));

  //     let i = nodejs_workers4.send(&b);
  //     println!("[3] {}", i);
  //   }
  // });

  let n = 10_000;
  let t = 1;

  // for t in 0..t {
  //   let nodejs = NodejsInstanceStdio::spawn();

  //   PROFILER.start(&format!("stdio {}", t));
  //   for i in 0..n {
  //   // for i in 0..1 {
  //     let nodejs = nodejs.clone();

  //     thread::sleep(Duration::from_nanos(1));

  //     thread::spawn(move || {
  //       let resp = nodejs.request(vec![0, 10]);
  //       // println!("{:?}", resp);
  //     });
  //   }
  //   PROFILER.lap(&format!("stdio {}", t));
  //   PROFILER.log_millis_total(&format!("stdio {}", t));
  // }

  // println!("");

  let nodejs = NodejsInstanceTcp::spawn(NodejsInstanceTcpOptions {
    workers: 2
  });

  // for t in 0..t {
  //   let nodejs = NodejsInstanceTcp::spawn(NodejsInstanceTcpOptions {
  //     workers: 2
  //   });

  //   PROFILER.start(&format!("tcp {}", t));
  //   for i in 0..n {
  //   // for i in 0..1 {
  //     let nodejs = nodejs.clone();

  //     thread::sleep(Duration::from_nanos(1));

  //     thread::spawn(move || {
  //       let resp = nodejs.request(vec![0, 10]);
  //       // println!("{:?}", resp);
  //     });
  //   }
  //   PROFILER.lap(&format!("tcp {}", t));

  //   PROFILER.log_millis_total(&format!("tcp {}", t));
  // }
  // for _ in 0..5 {
  //   let nodejs = nodejs.clone();
  //   thread::spawn(move || {
  //     let resp = nodejs.request(vec![0, 10]);
  //     thread::sleep(Duration::from_millis(500));
  //     println!("{:?}", resp);
  //   });
  // }

  // let nodejs1 = nodejs.clone();
  // thread::spawn(move || {
  //   let rx = nodejs1.subscribe();
  //   while let Ok(bytes) = rx.recv() {
  //     println!("{:?}", bytes)
  //   }

  //   thread::sleep(Duration::from_secs(2));
  // });

  // let nodejs2 = nodejs.clone();
  // thread::spawn(move || {
  //   for _ in 0..1 {
  //     let mut b = Vec::<u8>::from(&[1, 0]);
  //     b.extend(serde_json::to_vec("Hello").unwrap());
  //     b.push(10);

  //     nodejs2.send(b);
  //   }

  //   thread::sleep(Duration::from_secs(2));
  // });

  thread::sleep(Duration::from_secs(6));
  // nodejs.wait().unwrap();
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
