use std::fmt::Debug;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use super::Dependency;
use super::DependencyOptions;
use super::ResolveResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AdapterOutgoingRequest {
  Ping(AdapterOutgoingRequestPing),
  ResolverRegister(AdapterOutgoingRequestResolverRegister),
  ResolverLoadConfig(AdapterOutgoingRequestResolverLoadConfig),
  ResolverResolve(AdapterOutgoingRequestResolverResolve),
  TransformerRegister(AdapterOutgoingRequestTransformerRegister),
  TransformerLoadConfig(AdapterOutgoingRequestTransformerLoadConfig),
  TransformerTransform(AdapterOutgoingRequestTransformerTransform),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingRequestPing {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingRequestResolverRegister {
  pub specifier: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingRequestResolverLoadConfig {
  pub specifier: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingRequestResolverResolve {
  pub specifier: String,
  pub dependency: Dependency,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingRequestTransformerRegister {
  pub specifier: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingRequestTransformerLoadConfig {
  pub specifier: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingRequestTransformerTransform {
  pub specifier: String,
  pub file_path: PathBuf,
  pub kind: String,
  pub content: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AdapterOutgoingResponse {
  Err(String),
  Ping(AdapterOutgoingResponsePing),
  ResolverRegister(AdapterOutgoingResponseResolverRegister),
  ResolverLoadConfig(AdapterOutgoingResponseResolverLoadConfig),
  ResolverResolve(AdapterOutgoingResponseResolverResolve),
  TransformerRegister(AdapterOutgoingResponseTransformerRegister),
  TransformerLoadConfig(AdapterOutgoingResponseTransformerLoadConfig),
  TransformerTransform(AdapterOutgoingResponseTransformerTransform),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingResponsePing {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingResponseResolverRegister {
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingResponseResolverLoadConfig {
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingResponseResolverResolve {
  pub resolve_result: Option<ResolveResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingResponseTransformerRegister {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingResponseTransformerLoadConfig {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterOutgoingResponseTransformerTransform {
  pub content: Vec<u8>,
  pub kind: String,
  pub dependencies: Vec<DependencyOptions>,
}