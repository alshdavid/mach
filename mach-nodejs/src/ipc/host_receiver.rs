use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

use mach::kit::ipc::sync::IpcChild;
use mach::public::nodejs::NodejsClientRequest;
use mach::public::nodejs::NodejsClientRequestContext;
use mach::public::nodejs::NodejsClientResponse;
use mach::public::nodejs::NodejsClientResponseContext;

pub struct HostReceiver {
  pub on: Receiver<(NodejsClientRequest, Sender<NodejsClientResponse>)>,
}

impl HostReceiver {
  pub fn new() -> Self {
    let ipc_child_client = std::env::var("MACH_IPC_CHANNEL_1").unwrap().to_string();
    let (tx, rx) = channel::<(NodejsClientRequest, Sender<NodejsClientResponse>)>();
    
    thread::spawn(move || {
      let ipc_child_client = IpcChild::<NodejsClientResponseContext, NodejsClientRequestContext>::new(&ipc_child_client);
      let irx = ipc_child_client.subscribe();

      while let Ok(data) = irx.recv() {
        match data.1 {
          req => {
            let (tx_reply, rx_reply) = channel::<NodejsClientResponse>();
            tx.send((req, tx_reply)).unwrap();
            let response = rx_reply.recv().unwrap();
            ipc_child_client.send(NodejsClientResponseContext(data.0, response));
          }
        }
      }
    });

    Self {
      on: rx,
    }
  }
}
