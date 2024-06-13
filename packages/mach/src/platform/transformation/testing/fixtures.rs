use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;

use crate::public::BundleBehavior;
use crate::public::DependencyPriority;
use crate::public::ExportSymbol;
use crate::public::ImportSymbol;
use crate::public::LinkingSymbol;
use crate::public::ReexportSymbol;
use crate::public::SpecifierType;

pub static SNAPSHOT_FILENAME: &str = "__graph.json";
pub static CARGO_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
pub static FIXTURES: Lazy<PathBuf> = Lazy::new(|| {
  CARGO_DIR
    .join("src")
    .join("platform")
    .join("transformation")
    .join("testing")
    .join("__fixtures")
});

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GraphSnapshot {
  pub config: GraphSnapshotConfig,
}

pub type GraphSnapshotConfig = HashMap<PathBuf, GraphSnapshotAsset>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GraphSnapshotAsset {
  pub file_path: PathBuf,
  pub kind: String,
  pub bundle_behavior: BundleBehavior,
  pub linking_symbols: Vec<LinkingSymbol>,
  pub imports: GraphSnapshotImport,
}

pub type Specifier = String;
pub type GraphSnapshotImport = HashMap<Specifier, DependencySnapshot>;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DependencySnapshot {
  pub resolves_to: PathBuf,
  pub specifier: String,
  pub specifier_type: SpecifierType,
  pub priority: DependencyPriority,
  pub linking_symbol: LinkingSymbol,
  pub bundle_behavior: BundleBehavior,
}

impl GraphSnapshot {
  pub fn load(config_path: &Path) -> Self {
    let config_str = fs::read_to_string(config_path).unwrap();
    let internal = serde_json::from_str::<GraphSnapshotConfig>(&config_str).unwrap();
    GraphSnapshot { config: internal }
  }

  pub fn save(
    filepath: &Path,
    config: GraphSnapshotConfig,
  ) {
    let config_str = serde_json::to_string_pretty(&config).unwrap();
    fs::write(filepath, &config_str).unwrap();
  }

  pub fn get_entries(&self) -> Vec<String> {
    self
      .config
      .get(&PathBuf::from(""))
      .unwrap()
      .imports
      .iter()
      .map(|(s, _)| s.clone())
      .collect::<Vec<String>>()
  }
}
