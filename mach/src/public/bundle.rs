use std::collections::HashSet;
use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use super::Asset;
use super::AssetId;
use super::InternalId;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BundleId(pub InternalId);

#[derive(Default, Clone)]
pub struct Bundle {
  pub id: BundleId,
  pub name: String,
  pub kind: String,
  pub assets: HashSet<AssetId>,
  pub entry_asset: Option<AssetId>,
}

impl Bundle {
  pub fn generate_name(
    &self,
    mut assets: Vec<&Asset>,
  ) -> String {
    // assets.sort_by(|a, b| a.file_path_relative.cmp(&b.file_path_relative));
    // let mut content_hashes = String::new();

    // for asset in assets {
    //   let result = format!(
    //     "{} {}\n",
    //     asset.file_path_relative.to_str().unwrap(),
    //     hash_sha_256(&asset.content)
    //   );
    //   content_hashes.push_str(&result);
    // }

    // let bundle_hash = truncate(&hash_string_sha_256(&content_hashes), ID_TRUNC);

    // if let Some(entry) = &self.entry_asset {
    //   let file_stem = entry.file_stem().unwrap().to_str().unwrap();
    //   return format!("{}.{}.{}", file_stem, bundle_hash, self.kind);
    // } else {
    //   return format!("{}.{}", bundle_hash, self.kind);
    // }
    return format!("{}.{}", self.id.0.to_string(), self.kind);
  }

  pub fn get_assets(&self) -> Vec<&AssetId> {
    return self.assets.iter().collect::<Vec<&AssetId>>();
  }
}

impl std::fmt::Debug for Bundle {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("Bundle")
      .field("id", &self.id.0)
      .field("name", &self.name)
      .field("kind", &self.kind)
      .field("assets", &self.assets)
      .field("entry_asset", &self.entry_asset)
      .finish()
  }
}

impl Debug for BundleId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "BundleId({})", &self.0.to_string())
  }
}
