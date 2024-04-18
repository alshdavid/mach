use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::thread;

use crate::kit::broadcast_channel::BroadcastChannel;
use crate::kit::ipc::sync::IpcHost;
use crate::public::nodejs::NodejsHostRequest;
use crate::public::nodejs::NodejsHostRequestContext;
use crate::public::nodejs::NodejsHostResponse;
use crate::public::nodejs::NodejsHostResponseContext;

#[derive(Clone)]
pub struct ChildReceiver {
  pub server_name: String,
  pub on: BroadcastChannel<(NodejsHostRequest, Sender<NodejsHostResponse>)>,
}

impl ChildReceiver {
  pub fn new() -> Self {
    let ipc_host_host = IpcHost::<NodejsHostResponseContext, NodejsHostRequestContext>::new();
    let server_name = ipc_host_host.server_name.clone();

    let trx = BroadcastChannel::<(NodejsHostRequest, Sender<NodejsHostResponse>)>::new();
    
    {
      let irx = ipc_host_host.subscribe();
      let trx = trx.clone();
      thread::spawn(move || {
        while let Ok(data) = irx.recv() {
          match data.1 {
            req => {
              let (tx_reply, rx_reply) = channel::<NodejsHostResponse>();
              trx.send((req, tx_reply)).unwrap();
              let response = rx_reply.recv().unwrap();
              ipc_host_host.send(NodejsHostResponseContext(data.0, response));
            }
          }
        }
      });
    }

    Self {
      server_name,
      on: trx,
    }
  }
}
