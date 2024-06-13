use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use petgraph::dot::Config;
use petgraph::dot::Dot;
use petgraph::prelude::*;
use petgraph::stable_graph::EdgeIndex;
use petgraph::stable_graph::EdgeReference;
use petgraph::stable_graph::Edges;
use petgraph::stable_graph::NodeIndex;
use petgraph::stable_graph::StableDiGraph;

use super::Asset;
use super::AssetId;
use super::Dependency;
use super::DependencyId;
use super::MachConfig;
use crate::core::config::ROOT_ASSET;
use crate::public::DependencyPriority;

pub type AssetGraphSync = Arc<RwLock<AssetGraph>>;

#[derive(Default, Clone, Debug)]
pub struct AssetGraph {
  node_index: HashMap<AssetId, NodeIndex>,
  edge_index: HashMap<DependencyId, EdgeIndex>,
  graph: StableDiGraph<Asset, Dependency>,
}

impl AssetGraph {
  pub fn add_asset(
    &mut self,
    asset: Asset,
  ) -> NodeIndex {
    let asset_id = asset.id.clone();
    let node_id = self.graph.add_node(asset);
    self.node_index.insert(asset_id, node_id.clone());
    node_id
  }

  pub fn add_dependency(
    &mut self,
    src: &AssetId,
    dest: &AssetId,
    dependency: Dependency,
  ) -> Result<(bool, EdgeIndex), String> {
    let Some(src_id) = self.node_index.get(&src) else {
      return Err(format!("Unable to find Source Asset with ID: {}", src));
    };
    let Some(dest_id) = self.node_index.get(&dest) else {
      return Err(format!("Unable to find Dest Asset with ID: {}", dest));
    };
    let dependency_id = dependency.id.clone();
    if let Some(edge_index) = self.edge_index.get(&dependency_id) {
      return Ok((false, edge_index.clone()));
    }
    let edge_id = self
      .graph
      .add_edge(src_id.clone(), dest_id.clone(), dependency);

    self.edge_index.insert(dependency_id, edge_id.clone());

    Ok((true, edge_id))
  }

  pub fn root_node(&self) -> NodeIndex {
    NodeIndex::from(0)
  }

  pub fn get_asset(
    &self,
    index: NodeIndex,
  ) -> Option<&Asset> {
    self.graph.node_weight(index)
  }

  pub fn get_dependency(
    &self,
    index: EdgeIndex,
  ) -> Option<&Dependency> {
    self.graph.edge_weight(index)
  }

  pub fn get_dependencies(
    &self,
    index: &NodeIndex,
  ) -> Edges<Dependency, Directed, u32> {
    self.graph.edges(index.clone())
  }

  pub fn as_graph(&self) -> &StableDiGraph<Asset, Dependency> {
    &self.graph
  }

  pub fn into_dot(
    &self,
    config: &MachConfig,
  ) -> String {
    let get_node_attribute = |_: &StableDiGraph<Asset, Dependency>,
                              (_, asset): (NodeIndex, &Asset)| {
      let mut label = String::from("ROOT");

      if asset.id == ROOT_ASSET.id {
        return format!("label = \"{}\" ", label);
      }
      label = asset.file_path_relative.to_str().unwrap().to_string();
      format!("label = \"{}\" ", label)
    };

    let get_edge_attribute =
      |_: &StableDiGraph<Asset, Dependency>, edge_ref: EdgeReference<Dependency>| -> String {
        let dependency = edge_ref.weight();
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
    format!("{:?}", dot)
  }
}
