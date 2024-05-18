use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use ipc_channel_adapter::host::sync::ChildSender;

use mach_bundler_core::public::AdapterIncomingRequest;
use mach_bundler_core::public::AdapterIncomingResponse;
use mach_bundler_core::public::AdapterOutgoingRequest;
use mach_bundler_core::public::AdapterOutgoingResponse;

/// NodejsWorker holds the channels used to talk to the
/// worker thread spawned by the Nodejs child process
pub struct NodejsWorker {
  pub child_sender: ChildSender<AdapterOutgoingRequest, AdapterOutgoingResponse>,
  pub child_receiver: Receiver<(AdapterIncomingRequest, Sender<AdapterIncomingResponse>)>,
}
