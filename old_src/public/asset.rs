use std::path::PathBuf;

use swc_core::ecma::ast::Module;

pub enum AssetKind {
    JavaScript,
    Style,
    Markup,
}

#[derive(Debug, Clone)]
pub struct Asset {
  pub file_path: PathBuf,
  pub file_path_relative: PathBuf,
  pub file_path_relative_hash: String,
  pub kind: AssetKind,
  pub ast: Option<Module>,
  pub source_content_hash: Option<String>,
}
