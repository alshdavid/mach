use super::AssetContents;
use super::AssetGraph;
use super::AssetMap;
use super::DependencyMap;

#[derive(Default)]
pub struct Compilation {
  asset_map: AssetMap,
  asset_contents: AssetContents,
  asset_graph: AssetGraph,
  dependency_map: DependencyMap,
}

impl Compilation {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }
}
