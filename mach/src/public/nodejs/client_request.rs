use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodejsClientRequest {
  Ping { id: u8 },
  ResolverRegister { id: u8, data: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodejsClientResponse {
  Ping,
  ResolverRegister,
}
