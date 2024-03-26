use std::collections::HashSet;
use std::path::PathBuf;

use crate::kit::hash::hash_path_buff_sha_256;
use crate::kit::hash::hash_sha_256;
use crate::kit::hash::hash_string_sha_256;
use crate::kit::hash::truncate;

use super::Asset;
use super::InternalId;
use super::ID_TRUNC;

#[derive(Clone, Default, Debug)]
pub struct BundleId(InternalId);

#[derive(Default, Clone)]
pub struct Bundle {
  pub id: BundleId,
  pub content_key: String,
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

  pub fn generate_name(
    &self,
    mut assets: Vec<&Asset>,
  ) -> String {
    assets.sort_by(|a, b| a.file_path_relative.cmp(&b.file_path_relative));
    let mut content_hashes = String::new();

    for asset in assets {
      let result = format!(
        "{} {}\n",
        asset.file_path_relative.to_str().unwrap(),
        hash_sha_256(&asset.content)
      );
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

impl std::fmt::Debug for Bundle {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("Bundle")
      .field("id", &self.id.0)
      .field("content_key", &self.content_key)
      .field("name", &self.name)
      .field("kind", &self.kind)
      .field("assets", &self.assets)
      .field("entry_asset", &self.entry_asset)
      .finish()
  }
}
