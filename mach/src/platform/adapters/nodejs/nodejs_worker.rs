use std::process::Command;
use std::process::Stdio;

use super::ChildReceiver;
use super::ChildSender;

pub struct NodejsWorker {
  pub child_sender: ChildSender,
  pub child_receiver: ChildReceiver,
}

// Todo improve performance
impl NodejsWorker {
  pub fn new() -> Self {
    let child_sender = ChildSender::new();
    let child_receiver = ChildReceiver::new();

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
      child_receiver,
    }
  }
}
