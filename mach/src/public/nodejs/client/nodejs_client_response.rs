use serde::Deserialize;
use serde::Serialize;

use crate::public::ResolveResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodejsClientResponse {
  Ping(NodejsClientResponsePing),
  ResolverRegister(NodejsClientResolverRegister),
  ResolverLoadConfig(()),
  ResolverResolve(NodejsClientResponseResolverResolve),
  TransformerRegister(()),
  TransformerLoadConfig(()),
  TransformerTransform(()),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResponsePing {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResolverRegister {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResponseResolverResolve {
  pub resolve_result: Option<ResolveResult>,
}
