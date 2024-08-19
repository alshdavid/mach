use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::types::Asset;
use crate::types::AssetId;

pub static ROOT_NODE: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(""));

pub static ROOT_ASSET: Lazy<Asset> = Lazy::new(|| Asset {
  id: AssetId::new(),
  name: "ROOT".to_string(),
  file_path_absolute: ROOT_NODE.clone(),
  file_path: ROOT_NODE.clone(),
  kind: Default::default(),
  content: Default::default(),
  bundle_behavior: Default::default(),
  linking_symbols: Default::default(),
});
