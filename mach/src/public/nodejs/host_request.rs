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
