use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use ipc_channel_adapter::host::sync::ChildSender;

use crate::public::AdapterIncomingRequest;
use crate::public::AdapterIncomingResponse;
use crate::public::AdapterOutgoingRequest;
use crate::public::AdapterOutgoingResponse;

/// NodejsWorker holds the channels used to talk to the
/// worker thread spawned by the Nodejs child process
pub struct NodejsWorker {
  pub child_sender: ChildSender<AdapterOutgoingRequest, AdapterOutgoingResponse>,
  pub child_receiver: Receiver<(AdapterIncomingRequest, Sender<AdapterIncomingResponse>)>,
}
