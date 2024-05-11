use std::process::Stdio;

use ipc_channel_adapter::host::asynch::ChildReceiver;
use ipc_channel_adapter::host::asynch::ChildSender;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedSender;

use super::NodejsWorker;
use crate::public::nodejs::NodejsClientRequest;
use crate::public::nodejs::NodejsClientResponse;
use crate::public::nodejs::NodejsHostRequest;
use crate::public::nodejs::NodejsHostResponse;

#[derive(Clone)]
pub struct NodejsInstance {
  tx_stdin: UnboundedSender<Vec<u8>>,
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
      .join("lib")
      .join("main.js");

    let mut command = Command::new("node");
    command.arg("--title");
    command.arg("mach_nodejs_worker");
    command.arg(entry);

    command.stderr(Stdio::inherit());
    command.stdout(Stdio::inherit());
    command.stdin(Stdio::piped());

    let mut child = command.spawn().unwrap();

    let (tx_stdin, mut rx_stdin) = unbounded_channel::<Vec<u8>>();

    let mut stdin = child.stdin.take().unwrap();

    tokio::spawn(async move {
      while let Some(mut bytes) = rx_stdin.recv().await {
        bytes.push(10);
        stdin.write(&bytes).await.unwrap();
      }
    });

    Self { tx_stdin }
  }

  pub async fn spawn_worker(&self) -> NodejsWorker {
    let child_sender = ChildSender::<NodejsClientRequest, NodejsClientResponse>::new();
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
