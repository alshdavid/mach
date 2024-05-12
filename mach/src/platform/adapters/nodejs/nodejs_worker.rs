use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use ipc_channel_adapter::host::sync::ChildSender;

use crate::public::nodejs::client::NodejsClientRequest;
use crate::public::nodejs::client::NodejsClientResponse;
use crate::public::nodejs::NodejsHostRequest;
use crate::public::nodejs::NodejsHostResponse;

/// NodejsWorker holds the channels used to talk to the 
/// worker thread spawned by the Nodejs child process
pub struct NodejsWorker {
  pub child_sender: ChildSender<NodejsClientRequest, NodejsClientResponse>,
  pub child_receiver: Receiver<(NodejsHostRequest, Sender<NodejsHostResponse>)>,
}
