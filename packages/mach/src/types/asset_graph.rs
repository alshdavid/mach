use anyhow::Context;
use once_cell::sync::Lazy;
/// Extending StableGraph with Asset & Dependency specifics
use petgraph::prelude::*;
use petgraph::stable_graph::Edges;
use petgraph::stable_graph::NodeIndices;
use petgraph::stable_graph::StableDiGraph;

use super::Asset;
use super::AssetId;
use super::Dependency;

pub type AssetGraph = StableDiGraph<Asset, Dependency>;

pub static ROOT_ASSET: Lazy<AssetId> = Lazy::new(|| AssetId::new(0));

impl AssetGraphExt for AssetGraph {
  fn add_asset(
    &mut self,
    asset: Asset,
  ) -> &mut Asset {
    let nx = self.add_node(asset);
    let asset = self.node_weight_mut(nx.clone()).unwrap();
    asset.id.set(nx).unwrap();
    asset
  }

  fn get_asset(
    &self,
    id: AssetId,
  ) -> Option<&Asset> {
    self.node_weight(id)
  }

  fn try_get_asset(
    &self,
    id: AssetId,
  ) -> anyhow::Result<&Asset> {
    self.get_asset(id).context("Asset does not exist")
  }

  fn get_assets(&self) -> NodeIndices<Asset> {
    self.node_indices()
  }

  fn get_asset_mut(
    &mut self,
    id: AssetId,
  ) -> Option<&mut Asset> {
    self.node_weight_mut(id)
  }

  fn add_dependency(
    &mut self,
    src: AssetId,
    dest: AssetId,
    dependency: Dependency,
  ) -> &mut Dependency {
    let ex = self.add_edge(src, dest, dependency);
    let dependency = self.edge_weight_mut(ex.clone()).unwrap();
    dependency.id.set(ex).unwrap();
    dependency
  }

  fn get_dependencies(
    &self,
    id: AssetId,
  ) -> Edges<Dependency, Directed, u32> {
    self.edges(id.clone())
  }
}

pub trait AssetGraphExt {
  fn add_asset(
    &mut self,
    asset: Asset,
  ) -> &mut Asset;
  fn get_asset(
    &self,
    id: AssetId,
  ) -> Option<&Asset>;
  fn try_get_asset(
    &self,
    id: AssetId,
  ) -> anyhow::Result<&Asset>;
  fn get_asset_mut(
    &mut self,
    id: AssetId,
  ) -> Option<&mut Asset>;
  fn get_assets(&self) -> NodeIndices<Asset>;
  fn add_dependency(
    &mut self,
    src: AssetId,
    dest: AssetId,
    dependency: Dependency,
  ) -> &mut Dependency;
  fn get_dependencies(
    &self,
    id: AssetId,
  ) -> Edges<Dependency, Directed, u32>;
}
