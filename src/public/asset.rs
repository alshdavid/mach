use std::collections::HashMap;
use std::path::PathBuf;

use swc_core::common::util::take::Take;
use swc_core::ecma::ast::Module;

pub type AssetId = String;

#[derive(Debug, Clone)]
pub struct Asset {
  pub id: AssetId,
  pub file_path: PathBuf,
  pub file_path_relative: PathBuf,
  pub ast: Module,
}

impl Asset {
  pub fn new(project_path: &PathBuf, file_path: &PathBuf) -> Self {
    let file_path_relative = pathdiff::diff_paths(file_path, project_path).unwrap();
    return Asset {
      id: Asset::generate_id(&project_path, &file_path),
      file_path_relative,
      file_path: file_path.clone(),
      ast: Module::dummy(),
    };
  }

  pub fn generate_id(project_path: &PathBuf, file_path: &PathBuf) -> String {
    let rel_path = pathdiff::diff_paths(file_path, project_path).unwrap();
    return truncate(&super::hash_path_buff_sha_256(&rel_path), 9);
    // return rel_path.to_str().unwrap().to_string();
  }
}

pub type AssetMap = HashMap<AssetId, Asset>;

fn truncate(s: &str, max_chars: usize) -> String {
  match s.char_indices().nth(max_chars) {
    None => s.to_string(),
    Some((idx, _)) => s[..idx].to_string(),
  }
}
