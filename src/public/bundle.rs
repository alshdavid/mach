use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use super::ImportSymbolType;

#[derive(Debug)]
pub struct Bundle {
  // pub used_exports: HashMap<PathBuf, HashSet<ImportSymbolType>>,
  pub assets: HashSet<PathBuf>,
  pub entry_asset: PathBuf,
}

impl Bundle {
//   pub fn add_used_export(&mut self, asset_id: &Path, export: ImportSymbolType) {
//     if let Some(used_exports) = self.used_exports.get_mut(asset_id) {
//       used_exports.insert(export.clone());
//     } else {
//       let mut used_exports = HashSet::<ImportSymbolType>::new();
//       used_exports.insert(export.clone());
//       self.used_exports.insert(asset_id.to_path_buf(), used_exports);
//     }
//   }
}

pub type Bundles = Vec<Bundle>;
