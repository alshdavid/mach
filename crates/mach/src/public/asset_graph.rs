use std::collections::HashMap;
use std::collections::HashSet;

use super::Asset;
use super::AssetId;

pub type AssetEdge = (AssetId, AssetId);

#[derive(Debug)]
pub struct AssetNode {
  id: String,
  asset_id: AssetId
}

#[derive(Debug)]
pub struct AssetGraph {
  edges: HashSet<AssetEdge>,
  nodes: HashMap<AssetId, AssetNode>,
}

impl AssetGraph {
  pub fn new() -> Self {
    return AssetGraph{
        edges: HashSet::new(),
        nodes: HashMap::new(),
    };                         
  }

  pub fn add_node(
    &mut self,
    asset: AssetNode,
  ) {
    self.nodes.insert(asset.id.clone(), asset);
  }

  pub fn add_edge(
    &mut self,
    from: AssetId,
    to: AssetId,
  ) {
    self.edges.insert((from, to));
  }
}