use std::io::Write;
use std::net::TcpListener;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use super::NodejsWorkerTcp;
use super::super::NodejsWorker;
use super::super::NodejsWorkerFactory;

// TODO https://crates.io/crates/arrow
// TODO https://github.com/mtth/avsc
// TODO Use TCP for windows and UNIX Domain (or named) sockets for MacOS and Linux

#[derive(Clone)]
pub struct NodejsInstanceTcp {
  tx_write: Sender<Vec<u8>>,
  child: Arc<Child>,
}

impl NodejsInstanceTcp {
  pub fn new() -> Self {
    let entry = std::env::current_exe()
      .unwrap()
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .join("nodejs")
      .join("src")
      .join("cmd")
      .join("tcp")
      .join("main.js");

    let mut command = Command::new("node");
    command.arg("--title");
    command.arg("nodejs_mach");
    command.arg(entry);

    command.stderr(Stdio::inherit());
    command.stdout(Stdio::inherit());
    command.stdin(Stdio::piped());

    let mut child = command.spawn().unwrap();

    let (tx_write, rx_write) = channel::<Vec<u8>>();
    let mut stdin = child.stdin.take().unwrap();
    thread::spawn(move || {
      while let Ok(bytes) = rx_write.recv() {
        if stdin.write(&bytes).is_err() {
          break;
        }
      }
    });

    return Self {
      tx_write,
      child: Arc::new(child),
    };
  }
}

impl NodejsWorkerFactory for NodejsInstanceTcp {
  fn spawn(&self) -> Arc<dyn NodejsWorker> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let mut bytes = serde_json::to_vec(&port).unwrap();
    bytes.push(10);
    self.tx_write.send(bytes).unwrap();

    let Ok((stream, _)) = listener.accept() else {
      panic!("Unable to connect");
    };

    Arc::new(NodejsWorkerTcp::new(stream))
  }
}

