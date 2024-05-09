use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodejsClientRequest {
  Ping,
  ResolverRegister(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodejsClientResponse {
  Ping,
  ResolverRegister,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientRequestContext(pub usize, pub NodejsClientRequest);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResponseContext(pub usize, pub NodejsClientResponse);
