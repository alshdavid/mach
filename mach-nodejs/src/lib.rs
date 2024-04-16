mod ipc;

use ipc::HostSender;
use mach::kit::ipc::sync::IpcChild;
use mach::public::nodejs::NodejsClientRequestContext;
use mach::public::nodejs::NodejsClientResponseContext;
use mach::public::nodejs::NodejsHostRequest;
use napi_derive::napi;

#[napi]
pub fn start() {
  let host_sender = HostSender::new();


  // let rx = ipc_child.subscribe();
  // while let Ok(data) = rx.recv() {
  //   match data.1 {
  //     NodejsClientRequest::Ping => {
  //       ipc_child.send(NodejsClientResponseContext(data.0, NodejsClientResponse::Ping));
  //     }
  //   }
  // }

  host_sender.send_blocking(NodejsHostRequest::Ping);
}

struct HostReceiver {
  ipc_child_client: IpcChild<NodejsClientResponseContext, NodejsClientRequestContext>
}

impl HostReceiver {
  pub fn new() -> Self {
    let ipc_child_client = std::env::var("MACH_IPC_CHANNEL_1").unwrap().to_string();
    let ipc_child_client = IpcChild::<NodejsClientResponseContext, NodejsClientRequestContext>::new(&ipc_child_client);
  
    Self {
      ipc_child_client
    }
  }
}
