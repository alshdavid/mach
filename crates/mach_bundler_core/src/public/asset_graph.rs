use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use petgraph::dot::Config;
use petgraph::dot::Dot;
use petgraph::graph::DiGraph;
use petgraph::graph::EdgeIndex;
use petgraph::graph::EdgeReference;
use petgraph::graph::NodeIndex;
use petgraph::graph::NodeReferences;
use petgraph::Graph;

use super::AssetId;
use super::AssetMap;
use super::DependencyId;
use super::DependencyMap;
use super::MachConfig;
use crate::platform::config::ROOT_ASSET;
use crate::public::dependency;
use crate::public::dependency_id;
use crate::public::DependencyPriority;

pub type AssetGraphSync = Arc<RwLock<AssetGraph>>;

#[derive(Default)]
pub struct AssetGraph {
  node_index: HashMap<AssetId, NodeIndex>,
  edge_index: HashMap<DependencyId, EdgeIndex>,
  graph: DiGraph<AssetId, DependencyId>,
}

impl AssetGraph {
  pub fn add_asset(
    &mut self,
    asset_id: AssetId,
  ) {
    let node_id = self.graph.add_node(asset_id.clone());
    self.node_index.insert(asset_id, node_id);
  }

  pub fn add_dependency(
    &mut self,
    src: &AssetId,
    dest: &AssetId,
    dependency_id: DependencyId,
  ) {
    let src_id = self.node_index.get(&src).unwrap();
    let dest_id = self.node_index.get(&dest).unwrap();
    let edge_id = self
      .graph
      .add_edge(src_id.clone(), dest_id.clone(), dependency_id.clone());

    self.edge_index.insert(dependency_id, edge_id);
  }

  pub fn into_dot(
    &self,
    config: &MachConfig,
    asset_map: &AssetMap,
    dependency_map: &DependencyMap,
  ) -> String {
    let get_node_attribute = |_: &Graph<AssetId, DependencyId>,
                              (_, asset_id): (NodeIndex, &AssetId)| {
      let mut label = String::from("ROOT");
      let asset = asset_map.get(asset_id).unwrap();

      if asset.id == ROOT_ASSET.id {
        return format!("label = \"{}\" ", label);
      }
      label = asset.file_path_relative.to_str().unwrap().to_string();
      format!("label = \"{}\" ", label)
    };

    let get_edge_attribute =
      |_: &Graph<AssetId, DependencyId>, edge_ref: EdgeReference<DependencyId>| -> String {
        let dependency = dependency_map.get(edge_ref.weight()).unwrap();
        let mut label = String::new();

        let mut specifier = dependency.specifier.clone();
        if dependency.specifier.starts_with("/") || dependency.specifier.starts_with("\\") {
          specifier = format!(
            "./{}",
            pathdiff::diff_paths(&PathBuf::from(&dependency.specifier), &config.project_root)
              .unwrap()
              .to_str()
              .unwrap()
          );
        }

        label += &format!("label = \"{}\" ", specifier);

        if let DependencyPriority::Lazy = dependency.priority {
          label += &format!("; style = \"dashed\" ")
        }

        label
      };

    let dot = Dot::with_attr_getters(
      &self.graph,
      &[Config::EdgeNoLabel, Config::NodeNoLabel],
      &get_edge_attribute,
      &get_node_attribute,
    );
    format!("{}", dot)
  }
}
