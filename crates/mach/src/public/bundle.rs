use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::platform::hash::{hash_string_sha_256, truncate};

use super::ID_TRUNC;

#[derive(Debug, Default)]
pub struct Bundle {
  pub id: String,
  pub name: String,
  pub kind: String,
  pub assets: HashSet<PathBuf>,
  pub entry_asset: PathBuf,
}

impl Bundle {
  pub fn new(entry_asset: &Path, kind: &str) -> Self {
    let mut bundle = Self {
      kind: kind.to_string(),
      ..Default::default()
    };

    bundle.update_entry(entry_asset);
    return bundle;
  }

  pub fn update_entry(&mut self, entry_asset: &Path) {
    let mut file_stem = String::new();
    let mut file_name = String::new();
    let mut file_extension = String::new();

    if let Some(ext) = entry_asset.extension() {
      file_extension = ext.to_str().unwrap().to_string();
    };

    if let Some(fname) = entry_asset.file_name() {
      file_name = fname.to_str().unwrap().to_string();
    } else {
      file_name = "".to_string();
    };

    if file_extension == "" {
      file_stem = file_name;
    } else {
      file_stem = file_name
        .replace(&format!(".{}", file_extension), "");
    }

    self.name = file_stem;
    self.id = truncate(&hash_string_sha_256(entry_asset.to_str().unwrap()), ID_TRUNC);
    self.entry_asset = entry_asset.to_path_buf();
  }
}

pub type Bundles = Vec<Bundle>;
pub type BundleGraph = HashMap<String, String>;
