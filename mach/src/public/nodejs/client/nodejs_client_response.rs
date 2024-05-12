use serde::Deserialize;
use serde::Serialize;

use crate::public::ResolveResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodejsClientResponse {
  Ping(NodejsClientResponsePing),
  ResolverRegister(NodejsClientResolverRegister),
  ResolverRun(NodejsClientResponseResolverRun),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResponsePing {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResolverRegister {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientResponseResolverRun {
  pub resolve_result: Option<ResolveResult>,
}
