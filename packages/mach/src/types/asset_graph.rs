use std::collections::HashMap;

use petgraph::prelude::*;
use petgraph::stable_graph::EdgeIndex;
use petgraph::stable_graph::Edges;
use petgraph::stable_graph::NodeIndex;
use petgraph::stable_graph::StableDiGraph;

use super::Asset;
use super::AssetId;
use super::Dependency;
use super::DependencyId;

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
  ) -> anyhow::Result<(bool, EdgeIndex)> {
    let Some(src_id) = self.node_index.get(&src) else {
      anyhow::bail!("Unable to find Source Asset with ID: {}", src);
    };
    let Some(dest_id) = self.node_index.get(&dest) else {
      anyhow::bail!("Unable to find Dest Asset with ID: {}", dest);
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

  pub fn get(
    &self,
    asset_id: &AssetId,
  ) -> Option<&Asset> {
    let nx = self.node_index.get(asset_id)?;
    self.get_with_nx(nx.clone())
  }

  pub fn get_with_nx(
    &self,
    index: NodeIndex,
  ) -> Option<&Asset> {
    self.graph.node_weight(index)
  }

  pub fn get_nx(
    &self,
    index: &AssetId,
  ) -> Option<&NodeIndex> {
    self.node_index.get(index)
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
}
