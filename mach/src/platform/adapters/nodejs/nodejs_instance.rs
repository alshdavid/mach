use std::io::Write;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;

use ipc_channel_adapter::host::sync::ChildReceiver;
use ipc_channel_adapter::host::sync::ChildSender;

use super::NodejsWorker;
use crate::public::nodejs::client::NodejsClientRequest;
use crate::public::nodejs::client::NodejsClientResponse;
use crate::public::nodejs::NodejsHostRequest;
use crate::public::nodejs::NodejsHostResponse;

#[derive(Clone)]
pub struct NodejsInstance {
  tx_stdin: Sender<Vec<u8>>,
  child: Arc<Child>,
}

/// NodejsInstance wraps the Node.js Process
impl NodejsInstance {
  pub fn new() -> Self {
    let entry = std::env::current_exe()
      .unwrap()
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .join("nodejs")
      .join("main.js");

    let mut command = Command::new("node");
    command.arg("--title");
    command.arg("mach_nodejs_worker");
    command.arg(entry);

    command.stderr(Stdio::inherit());
    command.stdout(Stdio::inherit());
    command.stdin(Stdio::piped());

    let mut child = command.spawn().unwrap();

    let (tx_stdin, rx_stdin) = channel::<Vec<u8>>();

    let mut stdin = child.stdin.take().unwrap();

    thread::spawn(move || {
      while let Ok(mut bytes) = rx_stdin.recv() {
        bytes.push(10);
        stdin.write(&bytes).unwrap();
      }
    });

    Self { 
      tx_stdin,
      child: Arc::new(child),
    }
  }

  pub fn spawn_worker(&self) -> NodejsWorker {
    let child_sender = ChildSender::<NodejsClientRequest, NodejsClientResponse>::new().unwrap();
    let (child_receiver, rx_child_receiver) =
      ChildReceiver::<NodejsHostRequest, NodejsHostResponse>::new().unwrap();

    let msg = format!(
      "{}&{}",
      child_sender.server_name, child_receiver.server_name
    );
    self.tx_stdin.send(msg.as_bytes().to_vec()).unwrap();

    NodejsWorker {
      child_sender,
      child_receiver: rx_child_receiver,
    }
  }
}
