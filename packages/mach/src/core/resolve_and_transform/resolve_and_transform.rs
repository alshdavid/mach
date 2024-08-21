use std::collections::HashMap;
use std::path::PathBuf;

use super::run_resolvers::run_resolvers;
use crate::core::resolve_and_transform::run_transformers::run_transformers;
use crate::types::Asset;
use crate::types::AssetGraphExt;
use crate::types::AssetId;
use crate::types::Compilation;
use crate::types::Dependency;
use crate::types::LinkingSymbol;

pub fn resolve_and_transform(
  Compilation {
    asset_graph,
    config,
    plugins,
    ..
  }: &mut Compilation
) -> anyhow::Result<()> {
  let mut queue = vec![];
  let root_asset = asset_graph.add_asset(Asset::default());

  // Add entries from config
  for entry in config.entries.iter() {
    queue.push(Dependency {
      id: Default::default(),
      specifier: entry.to_str().unwrap().to_string(),
      source_asset_path: config.project_root.clone(),
      resolve_from: config.project_root.clone(),
      source_asset_id: root_asset.id.get()?.clone(),
      specifier_type: Default::default(),
      priority: Default::default(),
      linking_symbol: LinkingSymbol::ImportDirect {
        specifier: entry.to_str().unwrap().to_string(),
      },
      bundle_behavior: Default::default(),
    });
  }

  // Avoid re-processing assets
  let mut completed_assets = HashMap::<PathBuf, AssetId>::new();

  while let Some(dependency) = queue.pop() {
    let resolve_result = run_resolvers(config, plugins, &dependency)?;

    if let Some(asset_id) = completed_assets.get(&resolve_result.file_path) {
      // Asset exists
      asset_graph.add_dependency(dependency.source_asset_id, asset_id.clone(), dependency);
      continue;
    };

    // New Asset
    let mut transformer_pipeline_result = run_transformers(config, plugins, &resolve_result)?;

    // Create new asset and add it to the
    let new_asset_id = {
      let asset = asset_graph.add_asset(Asset {
        id: Default::default(),
        name: transformer_pipeline_result.name,
        file_path_absolute: resolve_result.file_path.clone(),
        file_path: resolve_result.file_path_relative.clone(),
        kind: transformer_pipeline_result.kind,
        content: transformer_pipeline_result.content,
        bundle_behavior: transformer_pipeline_result.bundle_behavior,
        linking_symbols: transformer_pipeline_result.linking_symbols,
      });
      asset.id.get()?.clone()
    };

    asset_graph.add_dependency(dependency.source_asset_id.clone(), new_asset_id, dependency);

    completed_assets.insert(resolve_result.file_path.clone(), new_asset_id.clone());

    while let Some(dependency_options) = transformer_pipeline_result.dependencies.pop() {
      let new_dependency = Dependency {
        id: Default::default(),
        specifier: dependency_options.specifier.clone(),
        specifier_type: dependency_options.specifier_type,
        source_asset_path: resolve_result.file_path.clone(),
        source_asset_id: new_asset_id.clone(),
        resolve_from: resolve_result.file_path.clone(),
        priority: dependency_options.priority,
        linking_symbol: dependency_options.linking_symbol,
        bundle_behavior: dependency_options.bundle_behavior,
      };

      queue.push(new_dependency);
    }
  }

  Ok(())
}
