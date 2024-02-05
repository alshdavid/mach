use std::path::PathBuf;

use swc_core::ecma::ast::Program;

use crate::platform::hash::hash_path_buff_sha_256;
use crate::platform::hash::hash_string_sha_256;

pub type AssetId = String;

#[derive(Debug, Clone)]
pub struct Asset {
  pub id: AssetId,
  pub file_path: PathBuf,
  pub file_path_relative: PathBuf,
  pub file_path_relative_hash: String,
  pub source_content_hash: String,
  pub code: String,
}

impl Asset {
  pub fn new(
    root_path: &PathBuf,
    asset_filepath_absolute: &PathBuf,
    code: String,
  ) -> Self {
    let relative_path = pathdiff::diff_paths(asset_filepath_absolute, root_path).unwrap();
    let relative_path_hash = hash_path_buff_sha_256(&relative_path);
    let source_content_hash = hash_string_sha_256(&code);
    let id = relative_path.to_str().unwrap().to_string();

    return Asset {
      id,
      file_path: asset_filepath_absolute.clone(),
      file_path_relative: relative_path,
      file_path_relative_hash: relative_path_hash,
      code,
      source_content_hash,
    };
  }
}
