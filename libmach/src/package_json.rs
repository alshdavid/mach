use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageJson {
  pub name: Option<String>,
  pub version: Option<String>,
  pub types: Option<String>,
  pub main: Option<String>,
  pub module: Option<String>,
  pub exports: Option<PackageJsonExports>,
  pub workspaces: Option<String>,
  pub targets: Option<PackageJsonTargets>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PackageJsonExports {
  String(String),
  List(Vec<String>),
  Conditional(PackageJsonExportsPath),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PackageJsonExportsPath {
  String(HashMap<String, String>),
  Conditional(HashMap<String, HashMap<String, String>>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageJsonTargets {
  pub source: Option<String>,
  pub dist_dir: Option<String>,
}
