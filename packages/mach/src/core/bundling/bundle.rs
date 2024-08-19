use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::visit::NodeRef;

use crate::types::AssetId;
use crate::types::Bundle;
use crate::types::BundleId;
use crate::types::Compilation;

// Simple bundling algorithm that does not deduplicate assets or bundle split
pub fn bundle(c: &mut Compilation) -> anyhow::Result<()> {
  let asset_graph = c.asset_graph.as_graph();
  let root_node = c.asset_graph.root_node();

  let mut entries = Vec::<(AssetId, BundleId)>::new();

  for entry_index in asset_graph.neighbors(root_node) {
    let bundle = c
      .bundle_graph
      .get_bundle(c.bundle_graph.root_node())
      .unwrap();
    let asset = c.asset_graph.get_asset(entry_index).unwrap();
    entries.push((asset.id.clone(), bundle.id.clone()));
  }

  while let Some((entry_id, parent_bundle_id)) = entries.pop() {
    let parent_bundle_nx = c.bundle_graph.get_index(&parent_bundle_id).unwrap().clone();

    let entry_asset_nx = c
      .asset_graph
      .get_asset_from_asset_id(&entry_id)
      .unwrap()
      .clone();
    let entry_asset = c.asset_graph.get_asset(entry_asset_nx).unwrap();

    let mut bundle = Bundle {
      id: Default::default(),
      kind: entry_asset.kind.clone(),
      entry_asset: Some(entry_asset.id.clone()),
      assets: Default::default(),
    };

    let mut dependencies = Vec::<NodeIndex>::from(vec![entry_asset_nx]);

    while let Some(current_nx) = dependencies.pop() {
      let current_asset = c.asset_graph.get_asset(current_nx).unwrap();
      bundle.insert_asset(&current_asset);

      let mut dependencies_edges = c.asset_graph.as_graph().edges(current_nx);
      while let Some(edge) = dependencies_edges.next() {
        let dependency_nx = edge.target().id();
        let dependency_asset = c.asset_graph.get_asset(dependency_nx).unwrap();

        // Type change creates new bundle
        if bundle.kind != dependency_asset.kind {
          entries.push((dependency_asset.id.clone(), bundle.id.clone()));
          continue;
        }

        // TODO lazy imports

        // Add sync imports into current bundle
        dependencies.push(dependency_nx);
      }
    }

    if bundle.assets.len() != 0 {
      let nx = c.bundle_graph.add_bundle(bundle);
      c.bundle_graph.add_edge(&parent_bundle_nx, &nx)?;
    }
  }

  return Ok(());
}
