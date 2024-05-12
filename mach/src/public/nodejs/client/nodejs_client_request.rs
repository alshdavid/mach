use serde::Deserialize;
use serde::Serialize;

use crate::public::Dependency;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodejsClientRequest {
  Ping(NodejsClientRequestPing),
  ResolverRegister(NodejsClientRequestResolverRegister),
  ResolverRun(NodejsClientRequestResolverRun),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientRequestPing {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientRequestResolverRegister {
  pub specifier: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientRequestResolverRun {
  pub dependency: Dependency,
}
