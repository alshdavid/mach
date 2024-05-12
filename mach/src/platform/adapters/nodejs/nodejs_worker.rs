use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use ipc_channel_adapter::host::sync::ChildReceiver;
use ipc_channel_adapter::host::sync::ChildSender;

use crate::public::nodejs::client::NodejsClientRequest;
use crate::public::nodejs::client::NodejsClientResponse;
use crate::public::nodejs::NodejsHostRequest;
use crate::public::nodejs::NodejsHostResponse;

pub struct NodejsWorker {
  pub child_sender: ChildSender<NodejsClientRequest, NodejsClientResponse>,
  pub child_receiver: Receiver<(NodejsHostRequest, Sender<NodejsHostResponse>)>,
}
