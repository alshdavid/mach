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
pub struct NodejsClientRequestContext(pub usize, pub NodejsClientRequest);

#[derive(Clone, Serialize, Deserialize)]
pub struct NodejsClientResponseContext(pub usize, pub NodejsClientResponse);
