use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::public::Dependency;
use crate::public::DependencyOptions;
use crate::public::MutableAsset;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodejsClientRequest {
  Ping(NodejsClientRequestPing),
  ResolverRegister(NodejsClientRequestResolverRegister),
  ResolverLoadConfig(NodejsClientRequestResolverLoadConfig),
  ResolverResolve(NodejsClientRequestResolverResolve),
  TransformerRegister(NodejsClientRequestTransformerRegister),
  TransformerLoadConfig(NodejsClientRequestTransformerLoadConfig),
  TransformerTransform(NodejsClientRequestTransformerTransform),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientRequestPing {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientRequestResolverRegister {
  pub specifier: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientRequestResolverLoadConfig {
  pub specifier: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientRequestResolverResolve {
  pub specifier: String,
  pub dependency: Dependency,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientRequestTransformerRegister {
  pub specifier: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientRequestTransformerLoadConfig {
  pub specifier: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodejsClientRequestTransformerTransform {
  pub specifier: String,
  pub file_path: PathBuf,
  pub kind: String,
  pub content: Vec<u8>,
}
