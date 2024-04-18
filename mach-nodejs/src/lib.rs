mod ipc;

use ipc::HostReceiver;
use ipc::HostSender;
use mach::public::nodejs::NodejsClientResponse;
use mach::public::nodejs::NodejsHostRequest;
use napi_derive::napi;

#[napi]
pub fn start() {
  let host_sender = HostSender::new();
  let host_receiver = HostReceiver::new();

  let rx = host_sender.send(NodejsHostRequest::Ping);

  while let Ok((req, res)) = host_receiver.on.recv() {
    println!("From host {:?}", req);
    res.send(NodejsClientResponse::Ping).unwrap();
  }

  rx.recv().unwrap();
}

