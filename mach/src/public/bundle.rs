use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use crate::kit::hash::hash_path_buff_sha_256;
use crate::kit::hash::hash_sha_256;
use crate::kit::hash::hash_string_sha_256;
use crate::kit::hash::truncate;

use super::Asset;
use super::ID_TRUNC;

#[derive(Debug, Default)]
pub struct Bundle {
  pub id: String,
  pub name: String,
  pub kind: String,
  pub assets: HashSet<PathBuf>,
  pub entry_asset: Option<PathBuf>,
}

impl Bundle {
  pub fn generate_id(&self) -> String {
    let mut names = String::new();

    for input in &self.get_assets() {
      names.push_str(&hash_path_buff_sha_256(input));
    }

    let names_hash = truncate(&hash_string_sha_256(&names), ID_TRUNC);
    return names_hash;
  }

  pub fn generate_name(&self, mut assets: Vec<&Asset>) -> String {
    assets.sort_by(|a, b| a.file_path_rel.cmp(&b.file_path_rel));
    let mut content_hashes = String::new();

    for asset in assets {
      let result = format!("{} {}\n", asset.file_path_rel.to_str().unwrap(), hash_sha_256(&asset.content));
      content_hashes.push_str(&result);
    }

    let bundle_hash = truncate(&hash_string_sha_256(&content_hashes), ID_TRUNC);

    if let Some(entry) = &self.entry_asset {
      let file_stem = entry.file_stem().unwrap().to_str().unwrap();
      return format!("{}.{}.{}", file_stem, bundle_hash, self.kind);
    } else {
      return format!("{}.{}", bundle_hash, self.kind);
    }
  }

  pub fn get_assets(&self) -> Vec<&PathBuf> {
    let mut v = self.assets.iter().collect::<Vec<&PathBuf>>();
    v.sort();
    return v;
  }
}

pub type Bundles = Vec<Bundle>;
pub type BundleGraph = HashMap<String, String>;

