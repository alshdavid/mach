use super::AssetContents;
use super::AssetGraph;
use super::AssetMap;
use super::DependencyMap;

#[derive(Default)]
pub struct Compilation {
  pub asset_map: AssetMap,
  pub asset_contents: AssetContents,
  pub asset_graph: AssetGraph,
  pub dependency_map: DependencyMap,
}

impl Compilation {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }
}
