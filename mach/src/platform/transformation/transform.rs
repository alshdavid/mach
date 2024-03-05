use std::fs;

use crate::platform::plugins::PluginContainer;
use crate::platform::plugins::TransformerTarget;
use crate::public;
use crate::public::Asset;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Dependency;
use crate::public::DependencyMap;
use crate::public::DependencyOptions;
use crate::public::MutableAsset;
use crate::public::ENTRY_ASSET;

pub async fn link_and_transform(
  config: &public::Config,
  plugins: &PluginContainer,
  asset_map: &mut AssetMap,
  dependency_map: &mut DependencyMap,
  asset_graph: &mut AssetGraph,
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
    let file_path_rel =
      pathdiff::diff_paths(&resolve_result.file_path, &config.project_root).unwrap();
    // Resolve Done

    // Dependency Graph
    let dependency_source_path = dependency.resolve_from_rel.clone();
    let dependency_bundle_behavior = dependency.bundle_behavior.clone();
    let dependency_id = dependency_map.insert(dependency);
    asset_graph.add_edge(
      dependency_source_path.clone(),
      (dependency_id, file_path_rel.clone()),
    );
    if asset_map.contains_key(&file_path_rel) {
      continue;
    }
    // Dependency Graph Done

    // Transformation
    let mut file_target = TransformerTarget::new(&resolve_result.file_path);

    let mut content =
      fs::read(&resolve_result.file_path).map_err(|_| "Unable to read file".to_string())?;
    let mut asset_dependencies = Vec::<DependencyOptions>::new();
    let mut asset_kind = file_target.file_extension.clone();

    let mut mutable_asset = MutableAsset::new(
      resolve_result.file_path.clone(),
      &mut asset_kind,
      &mut content,
      &mut asset_dependencies,
    );

    let (mut pattern, mut transformers) = plugins.transformers.get(&file_target)?;

    let mut i = 0;
    while i != transformers.len() {
      let Some(transformer) = transformers.get(i) else {
        break;
      };

      transformer.transform(&mut mutable_asset, &config).await?;

      // If the file type and pattern changes restart transformers
      if *mutable_asset.kind != file_target.file_extension {
        file_target.update(mutable_asset.kind);

        let (new_pattern, new_transformers) = plugins.transformers.get(&file_target)?;
        // Use new transformers if they are different to current ones
        if new_pattern != pattern {
          transformers = new_transformers;
          pattern = new_pattern;
          i = 0;
          continue;
        }
      }

      i += 1;
    }

    asset_map.insert(Asset {
      name: file_target.file_stem.clone(),
      file_path: resolve_result.file_path.clone(),
      file_path_rel: file_path_rel.clone(),
      content,
      kind: asset_kind,
      bundle_behavior: dependency_bundle_behavior,
    });
    // Transformation Done

    // Add new items to the queue
    while let Some(dependency_options) = asset_dependencies.pop() {
      queue.push(Dependency {
        id: String::new(),
        specifier: dependency_options.specifier.clone(),
        specifier_type: dependency_options.specifier_type,
        is_entry: false,
        source_path: resolve_result.file_path.clone(),
        resolve_from: resolve_result.file_path.clone(),
        resolve_from_rel: file_path_rel.clone(),
        priority: dependency_options.priority,
        imported_symbols: dependency_options.imported_symbols,
        bundle_behavior: dependency_options.bundle_behavior,
      });
    }
  }

  Ok(())
}