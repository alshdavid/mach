use std::path::PathBuf;

use crate::platform::hash::hash_path_buff_sha_256;
use crate::platform::hash::hash_string_sha_256;

pub type AssetId = String;

#[derive(Debug, Clone)]
pub struct Asset {
  pub id: AssetId,
  pub file_path: PathBuf,
  pub code: String,
}

impl Asset {
  pub fn generate_id(
    root_path: &PathBuf,
    asset_filepath_absolute: &PathBuf,
    code: &str,
  ) -> String {
    let relative_path = pathdiff::diff_paths(asset_filepath_absolute, root_path).unwrap();
    let relative_path_hash = hash_path_buff_sha_256(&relative_path);
    let source_content_hash = hash_string_sha_256(&code);
    return hash_string_sha_256(&format!("{}{}", relative_path_hash, source_content_hash));
  }
}
