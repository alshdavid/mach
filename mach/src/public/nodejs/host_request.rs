use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodejsHostRequest {
  Ping,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodejsHostResponse {
  Ping,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsHostRequestContext(pub usize, pub NodejsHostRequest);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsHostResponseContext(pub usize, pub NodejsHostResponse);
