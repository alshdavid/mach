use std::path::PathBuf;

use crate::platform::hash::hash_path_buff_sha_256;

pub type AssetId = String;

#[derive(Debug, Clone)]
pub struct Asset {
  pub id: AssetId,
  pub file_path: PathBuf,
  pub content: Vec<u8>,
}

impl Asset {
  pub fn generate_id(
    root_path: &PathBuf,
    asset_filepath_absolute: &PathBuf,
  ) -> String {
    let relative_path = pathdiff::diff_paths(asset_filepath_absolute, root_path).unwrap();
    let relative_path_hash = hash_path_buff_sha_256(&relative_path);
    return relative_path_hash;
  }
}
