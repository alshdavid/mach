use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::ExportSymbol;

#[derive(Debug)]
pub struct Bundle {
  pub export_symbols: HashMap<PathBuf, HashSet<ExportSymbol>>,
  pub assets: HashSet<PathBuf>,
  pub entry_asset: PathBuf,
}

impl Bundle {
  pub fn insert_export_symbol(&mut self, asset_id: &Path, export_symbol: ExportSymbol) {
    if let Some(export_symbols) = self.export_symbols.get_mut(asset_id) {
      export_symbols.insert(export_symbol);
    } else {
      let export_symbols = HashSet::<ExportSymbol>::from([export_symbol]);
      self.export_symbols.insert(asset_id.to_path_buf(), export_symbols);
    }
  }
}

pub type Bundles = Vec<Bundle>;
