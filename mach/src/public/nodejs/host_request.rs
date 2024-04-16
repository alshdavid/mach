use ipc_channel::ipc::IpcSender;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize)]
pub enum NodejsHostRequest {
  Ping,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum NodejsHostResponse {
  Ping,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NodejsHostRequestContext(pub usize, pub NodejsHostRequest);

#[derive(Clone, Serialize, Deserialize)]
pub struct NodejsHostResponseContext(pub usize, pub NodejsHostResponse);
