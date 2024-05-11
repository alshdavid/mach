use std::process::Command;
use std::process::Stdio;

use ipc_channel_adapter::host::asynch::ChildReceiver;
use ipc_channel_adapter::host::asynch::ChildSender;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::oneshot::Sender as OneshotSender;

use crate::public::nodejs::NodejsHostRequest;
use crate::public::nodejs::NodejsHostResponse;
use crate::public::nodejs::NodejsClientRequest;
use crate::public::nodejs::NodejsClientResponse;

pub struct NodejsWorker {
  pub child_sender: ChildSender<NodejsClientRequest, NodejsClientResponse>,
  pub child_receiver: UnboundedReceiver<(NodejsHostRequest, OneshotSender<NodejsHostResponse>)>,
}

// Todo improve performance
impl NodejsWorker {
  pub fn new() -> Self {
    let child_sender = ChildSender::new();
    let (child_receiver, rx_child_receiver) = ChildReceiver::<NodejsHostRequest, NodejsHostResponse>::new().unwrap();

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
    command.env("MACH_IPC_CHANNEL_1", &child_sender.server_name);
    command.env("MACH_IPC_CHANNEL_2", &child_receiver.server_name);

    command.stderr(Stdio::inherit());
    command.stdout(Stdio::inherit());
    command.stdin(Stdio::piped());

    command.spawn().unwrap();

    Self {
      child_sender,
      child_receiver: rx_child_receiver,
    }
  }
}
