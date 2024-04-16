use mach::kit::ipc::sync::IpcChild;
use mach::public::nodejs::NodejsClientRequest;
use mach::public::nodejs::NodejsClientResponse;
use mach::public::nodejs::NodejsRequestContext;
use mach::public::nodejs::NodejsResponseContext;
use napi_derive::napi;

#[napi]
pub fn start() {
  let mach_ipc_channel = std::env::var("MACH_IPC_CHANNEL").unwrap().to_string();

  let ipc_child = IpcChild::<NodejsResponseContext, NodejsRequestContext>::new(&mach_ipc_channel);

  let rx = ipc_child.subscribe();
  while let Ok(data) = rx.recv() {
    match data.1 {
      NodejsClientRequest::Ping => {
        ipc_child.send(NodejsResponseContext(data.0, NodejsClientResponse::Ping));
      }
    }
  }
}
