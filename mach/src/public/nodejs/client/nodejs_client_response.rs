use serde::Deserialize;
use serde::Serialize;

use crate::public::DependencyOptions;
use crate::public::ResolveResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodejsClientResponse {
  Err(String),
  Ping(NodejsClientResponsePing),
  ResolverRegister(NodejsClientResolverRegister),
  ResolverLoadConfig(NodejsClientResolverLoadConfig),
  ResolverResolve(NodejsClientResponseResolverResolve),
  TransformerRegister(NodejsClientResponseTransformerRegister),
  TransformerLoadConfig(NodejsClientResponseTransformerLoadConfig),
  TransformerTransform(NodejsClientResponseTransformerTransform),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResponsePing {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResolverLoadConfig {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResolverRegister {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResponseResolverResolve {
  pub resolve_result: Option<ResolveResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResponseTransformerRegister {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResponseTransformerLoadConfig {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResponseTransformerTransform {
  pub content: Vec<u8>,
  pub kind: String,
  pub dependencies: Vec<DependencyOptions>,
}
