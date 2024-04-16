use ipc_channel::ipc::IpcSender;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize)]
pub enum NodejsClientRequest {
  Ping,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum NodejsClientResponse {
  Ping,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NodejsRequestContext(pub usize, pub NodejsClientRequest);

#[derive(Clone, Serialize, Deserialize)]
pub struct NodejsResponseContext(pub usize, pub NodejsClientResponse);
