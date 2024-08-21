use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::visit::NodeRef;

use crate::types::AssetGraphExt;
use crate::types::AssetId;
use crate::types::Bundle;
use crate::types::BundleGraphExt;
use crate::types::BundleId;
use crate::types::Compilation;
use crate::types::ROOT_ASSET;
use crate::types::ROOT_BUNDLE;

// Simple bundling algorithm that does not deduplicate assets or bundle split
pub fn bundle(
  Compilation {
    bundle_graph,
    asset_graph,
    ..
  }: &mut Compilation
) -> anyhow::Result<()> {
  bundle_graph.add_bundle(Bundle::default()); // Root bundle

  let mut entries = Vec::<(AssetId, BundleId)>::new();

  for entry_index in asset_graph.neighbors(ROOT_ASSET.clone()) {
    let bundle = bundle_graph.try_get_bundle(ROOT_BUNDLE.clone())?;

    let asset = asset_graph.try_get_asset(entry_index)?;
    entries.push((asset.id.get()?.clone(), bundle.id.get()?.clone()));
  }

  while let Some((entry_id, parent_bundle_id)) = entries.pop() {
    let parent_bundle_id = {
      let parent_bundle = bundle_graph.try_get_bundle(parent_bundle_id)?;
      parent_bundle.id.get()?.clone()
    };
    let entry_asset = asset_graph.get_asset(entry_id).unwrap();

    let bundle = bundle_graph.add_bundle(Bundle {
      id: Default::default(),
      kind: entry_asset.kind.clone(),
      entry_asset: Some(entry_asset.id.get()?.clone()),
      assets: Default::default(),
    });

    let mut dependencies = Vec::<NodeIndex>::from(vec![entry_asset.id.get()?.clone()]);

    while let Some(current_nx) = dependencies.pop() {
      let current_asset = asset_graph.get_asset(current_nx).unwrap();
      bundle.insert_asset(&current_asset)?;

      let mut dependencies_edges = asset_graph.edges(current_nx);
      while let Some(edge) = dependencies_edges.next() {
        let dependency_nx = edge.target().id();
        let dependency_asset = asset_graph.get_asset(dependency_nx).unwrap();

        // Type change creates new bundle
        if bundle.kind != dependency_asset.kind {
          entries.push((dependency_asset.id.get()?.clone(), bundle.id.get()?.clone()));
          continue;
        }

        // TODO lazy imports

        // Add sync imports into current bundle
        dependencies.push(dependency_nx);
      }
    }

    if bundle.assets.len() != 0 {
      let current_bundle_id = bundle.id.get()?.clone();
      bundle_graph.add_edge(parent_bundle_id, current_bundle_id, ());
    }
  }

  return Ok(());
}
