use crate::public::Asset;
use crate::public::AssetGraphSync;
use crate::public::AssetId;
use crate::public::AssetMapSync;
use crate::public::Dependency;
use crate::public::DependencyMapSync;
use crate::public::ResolveResult;

pub fn run_update_graph(
  asset_map: &AssetMapSync,
  asset_graph: &AssetGraphSync,
  dependency_map: &DependencyMapSync,
  dependency: Dependency,
  resolve_result: &ResolveResult,
) -> Result<(AssetId, bool), String> {
  let dependency_id = dependency.id.clone();
  let source_asset = dependency.source_asset.clone();

  // Add placeholder Asset to prevent future transformations
  let (asset_id, inserted) = if let Ok(mut asset_map) = asset_map.write() {
    asset_map.get_or_insert(Asset {
      file_path_absolute: resolve_result.file_path.clone(),
      bundle_behavior: dependency.bundle_behavior.clone(),
      ..Default::default()
    })
  } else {
    return Err("AssetMapMutexError".to_string());
  };

  if let Ok(mut dependency_map) = dependency_map.write() {
    dependency_map.insert(dependency);
  } else {
    return Err("DependencyMapMutexError".to_string());
  };

  if let Ok(mut asset_graph) = asset_graph.write() {
    asset_graph.add_edge(
      source_asset.clone(),
      asset_id.clone(),
      dependency_id.clone(),
    );
  } else {
    return Err("AssetGraphMutexError".to_string());
  };

  Ok((asset_id, inserted))
}
