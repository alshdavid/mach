use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::public::Dependency;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoadPluginRequest {
  pub plugin_key: String,
  pub specifier: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RunResolverRequest {
  pub plugin_key: String,
  pub dependency: Dependency,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RunResolverResponse {
  pub file_path: Option<PathBuf>,
}
