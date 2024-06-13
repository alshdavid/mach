use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use petgraph::visit::EdgeRef;
use petgraph::visit::NodeRef;
use serde::Deserialize;
use serde::Serialize;

use crate::platform::transformation::build_asset_graph;
use crate::platform::transformation::testing::utils::setup_root;
use crate::platform::transformation::testing::utils::SNAPSHOT_FILENAME;
use crate::public::BundleBehavior;
use crate::public::DependencyPriority;
use crate::public::ExportSymbol;
use crate::public::ImportSymbol;
use crate::public::LinkingSymbol;
use crate::public::ReexportSymbol;
use crate::public::SpecifierType;

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

pub fn generate_project_snapshot(project_root: &Path) {
  let snapshot_path = project_root.join(SNAPSHOT_FILENAME);

  let (mach_config, plugins, mut c) = setup_root(&project_root, &["./index.js"]);

  if let Err(msg) = build_asset_graph(mach_config.clone(), plugins, &mut c) {
    println!("{msg}");
    panic!()
  };

  let mut snapshot = GraphSnapshotConfig::default();
  let graph = c.asset_graph.as_graph();

  for node_index in graph.node_indices().into_iter() {
    let source_asset = c.asset_graph.get_asset(node_index).unwrap();

    let mut snapshot_imports = GraphSnapshotImport::default();

    let mut edges = c.asset_graph.get_dependencies(&node_index);

    while let Some(edge) = edges.next() {
      let dest_asset = c.asset_graph.get_asset(edge.target().id()).unwrap();
      let dependency = edge.weight();

      snapshot_imports.insert(
        dependency.specifier.clone(),
        DependencySnapshot {
          resolves_to: dest_asset.file_path_relative.clone(),
          specifier: dependency.specifier.clone(),
          specifier_type: dependency.specifier_type.clone(),
          priority: dependency.priority.clone(),
          linking_symbol: dependency.linking_symbol.clone(),
          bundle_behavior: dependency.bundle_behavior.clone(),
        },
      );
    }

    let snapshot_entry = GraphSnapshotAsset {
      file_path: source_asset.file_path_relative.clone(),
      linking_symbols: source_asset.linking_symbols.clone(),
      kind: source_asset.kind.clone(),
      bundle_behavior: source_asset.bundle_behavior.clone(),
      imports: snapshot_imports,
    };

    snapshot.insert(source_asset.file_path_relative.clone(), snapshot_entry);
  }

  GraphSnapshot::save(&snapshot_path, snapshot);
}
