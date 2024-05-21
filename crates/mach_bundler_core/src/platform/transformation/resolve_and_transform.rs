use std::collections::HashMap;
use std::path::PathBuf;

use super::run_resolvers::run_resolvers;
use crate::platform::config::PluginContainerSync;
use crate::platform::config::ROOT_ASSET;
use crate::platform::config::ROOT_NODE;
use crate::platform::transformation::run_transformers::run_transformers;
use crate::public::Asset;
use crate::public::AssetId;
use crate::public::Compilation;
use crate::public::Dependency;
use crate::public::DependencyId;
use crate::public::MachConfigSync;

pub fn resolve_and_transform(
  config: MachConfigSync,
  plugins: PluginContainerSync,
  mut compilation: Compilation,
) -> Result<(), String> {
  let mut queue = vec![];

  compilation
    .asset_map
    .insert(ROOT_ASSET.id.clone(), ROOT_ASSET.clone());

  compilation.asset_graph.add_asset(ROOT_ASSET.id.clone());

  for entry in config.entries.iter() {
    queue.push(Dependency {
      id: DependencyId::new(entry.to_str().unwrap()),
      specifier: entry.to_str().unwrap().to_string(),
      is_entry: true,
      source_path: config.project_root.clone(),
      resolve_from: config.project_root.clone(),
      source_asset: ROOT_ASSET.id.clone(),
      ..Dependency::default()
    });
  }

  let mut completed_assets = HashMap::<PathBuf, AssetId>::new();

  while let Some(dependency) = queue.pop() {
    dbg!(&dependency);
    compilation
      .dependency_map
      .insert(dependency.id.clone(), dependency.clone());

    let resolve_result = run_resolvers(&config, &plugins, &dependency)?;

    if let Some(asset_id) = completed_assets.get(&resolve_result.file_path) {
      compilation.asset_graph.add_dependency(
        &dependency.source_asset,
        &asset_id,
        dependency.id.clone(),
      );
      continue;
    };

    let mut new_asset = Asset {
      id: AssetId::new(resolve_result.file_path_relative.to_str().unwrap()),
      file_path_absolute: resolve_result.file_path.clone(),
      file_path_relative: resolve_result.file_path_relative.clone(),
      bundle_behavior: dependency.bundle_behavior.clone(),
      ..Asset::default()
    };
    completed_assets.insert(resolve_result.file_path.clone(), new_asset.id.clone());

    compilation.asset_graph.add_asset(new_asset.id.clone());

    compilation
      .asset_graph
      .add_dependency(&dependency.source_asset, &new_asset.id, dependency.id);

    let mut asset_dependencies =
      run_transformers(&config, &plugins, &mut new_asset, &resolve_result)?;

    while let Some(dependency_options) = asset_dependencies.pop() {
      let new_dependency = Dependency {
        id: DependencyId::new(&dependency_options.specifier),
        specifier: dependency_options.specifier.clone(),
        specifier_type: dependency_options.specifier_type,
        is_entry: false,
        source_path: resolve_result.file_path.clone(),
        source_asset: new_asset.id.clone(),
        resolve_from: resolve_result.file_path.clone(),
        priority: dependency_options.priority,
        imported_symbols: dependency_options.imported_symbols,
        bundle_behavior: dependency_options.bundle_behavior,
        ..Default::default()
      };

      queue.push(new_dependency);
    }

    compilation
      .asset_map
      .insert(new_asset.id.clone(), new_asset);
  }

  println!(
    "{}",
    compilation
      .asset_graph
      .into_dot(&config, &compilation.asset_map, &compilation.dependency_map)
  );

  Ok(())
}
