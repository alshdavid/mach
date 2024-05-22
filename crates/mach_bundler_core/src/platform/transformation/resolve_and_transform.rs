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

  compilation.asset_graph.add_asset(ROOT_ASSET.clone());

  for entry in config.entries.iter() {
    queue.push(Dependency {
      id: DependencyId::new(entry.to_str().unwrap()),
      specifier: entry.to_str().unwrap().to_string(),
      is_entry: true,
      source_asset_path: config.project_root.clone(),
      resolve_from: config.project_root.clone(),
      source_asset_id: ROOT_ASSET.id.clone(),
      ..Dependency::default()
    });
  }

  let mut completed_assets = HashMap::<PathBuf, AssetId>::new();

  while let Some(dependency) = queue.pop() {
    let resolve_result = run_resolvers(&config, &plugins, &dependency)?;

    if let Some(asset_id) = completed_assets.get(&resolve_result.file_path) {
      compilation.asset_graph.add_dependency(
        &dependency.source_asset_id.clone(),
        &asset_id,
        dependency,
      )?;
      continue;
    };

    let new_asset_id = AssetId::new(resolve_result.file_path_relative.to_str().unwrap());
    let mut new_asset = Asset {
      id: new_asset_id.clone(),
      file_path_absolute: resolve_result.file_path.clone(),
      file_path_relative: resolve_result.file_path_relative.clone(),
      bundle_behavior: dependency.bundle_behavior.clone(),
      ..Asset::default()
    };
    completed_assets.insert(resolve_result.file_path.clone(), new_asset_id.clone());

    let mut asset_dependencies =
      run_transformers(&config, &plugins, &mut new_asset, &resolve_result)?;

    while let Some(dependency_options) = asset_dependencies.pop() {
      let new_dependency = Dependency {
        id: DependencyId::new(&dependency_options.specifier),
        specifier: dependency_options.specifier.clone(),
        specifier_type: dependency_options.specifier_type,
        is_entry: false,
        source_asset_path: resolve_result.file_path.clone(),
        source_asset_id: new_asset.id.clone(),
        resolve_from: resolve_result.file_path.clone(),
        priority: dependency_options.priority,
        imported_symbols: dependency_options.imported_symbols,
        bundle_behavior: dependency_options.bundle_behavior,
        ..Default::default()
      };

      queue.push(new_dependency);
    }

    compilation.asset_graph.add_asset(new_asset);

    compilation.asset_graph.add_dependency(
      &dependency.source_asset_id.clone(),
      &new_asset_id.clone(),
      dependency,
    )?;
  }

  println!("{}", compilation.asset_graph.into_dot(&config));

  Ok(())
}
