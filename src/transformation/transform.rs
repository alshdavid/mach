use std::fs;

use crate::plugins::PluginContainer;
use crate::public;
use crate::public::Asset;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Dependency;
use crate::public::DependencyMap;
use crate::public::DependencyOptions;
use crate::public::ExportSymbol;
use crate::public::MutableAsset;
use crate::public::ENTRY_ASSET;

pub async fn transform(
  config: &public::Config,
  asset_map: &mut AssetMap,
  dependency_map: &mut DependencyMap,
  asset_graph: &mut AssetGraph,
  plugins: &PluginContainer,
) -> Result<(), String> {
  let mut queue = Vec::<Dependency>::new();

  // Entry Asset
  queue.push(Dependency {
    specifier: config.entry_point.to_str().unwrap().to_string(),
    is_entry: true,
    source_path: ENTRY_ASSET.clone(),
    resolve_from: ENTRY_ASSET.clone(),
    ..Dependency::default()
  });

  while let Some(dependency) = queue.pop() {
    // Resolve Start
    let resolve_result = 'block: {
      for resolver in &plugins.resolvers {
        if let Some(resolve_result) = resolver.resolve(&dependency).await? {
          break 'block resolve_result;
        }
      }
      return Err("Unable to resolve file".to_string());
    };
    // Resolve Done

    // Dependency Graph
    let dependency_source_path = dependency.source_path.clone();
    let dependency_bundle_behavior = dependency.bundle_behavior.clone();
    let dependency_id = dependency_map.insert(dependency);
    asset_graph.add_edge(
      dependency_source_path.clone(),
      (dependency_id, resolve_result.file_path.clone()),
    );
    if asset_map.contains_key(&resolve_result.file_path) {
      continue;
    }
    // Dependency Graph Done

    // Transformation
    let mut content =
      fs::read(&resolve_result.file_path).map_err(|_| "Unable to read file".to_string())?;

    let mut dependencies = Vec::<DependencyOptions>::new();
    let mut exports = Vec::<ExportSymbol>::new();

    let mut mutable_asset = MutableAsset::new(
      resolve_result.file_path.clone(),
      &mut content,
      &mut dependencies,
      &mut exports,
    );

    let Some(transformers) = plugins.transformers.get(&resolve_result.file_path) else {
      return Err(format!(
        "No transformer found {:?}",
        resolve_result.file_path
      ));
    };

    for transformer in transformers {
      transformer.transform(&mut mutable_asset, &config).await?;
    }

    asset_map.insert(Asset {
      file_path: resolve_result.file_path.clone(),
      content,
      bundle_behavior: dependency_bundle_behavior,
      exports,
    });
    // Transformation Done

    // Add new items to the queue
    while let Some(dependency_options) = dependencies.pop() {
      queue.push(Dependency {
        specifier: dependency_options.specifier.clone(),
        specifier_type: dependency_options.specifier_type,
        is_entry: false,
        source_path: resolve_result.file_path.clone(),
        resolve_from: resolve_result.file_path.clone(),
        priority: dependency_options.priority,
        imported_symbols: dependency_options.imported_symbols,
        bundle_behavior: dependency_options.bundle_behavior,
      });
    }
  }

  Ok(())
}
