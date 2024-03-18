use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use libmach::Config;
use libmach::Dependency;
use libmach::DependencyOptions;

#[derive(Serialize, Clone, Debug)]
pub struct LoadPluginRequest {
  pub plugin_key: String,
  pub specifier: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct RunResolverRequest {
  pub plugin_key: String,
  pub dependency: Dependency,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RunResolverResponse {
  pub file_path: Option<PathBuf>,
}

#[derive(Serialize, Clone, Debug)]
pub struct RunTransformerRequest {
  pub plugin_key: String,
  pub config: Config,
  pub file_path: PathBuf,
  pub kind: String,
  pub code: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RunTransformerResponse {
  pub updated: bool,
  pub dependencies: Vec<DependencyOptions>,
  pub code: String,
}
