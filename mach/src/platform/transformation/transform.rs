use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::task::JoinSet;

use crate::platform::config::PluginContainer;
use crate::platform::config::TransformerTarget;
use crate::platform::constants::ENTRY_ASSET;
use libmach;
use libmach::Asset;
use libmach::AssetGraph;
use libmach::AssetMap;
use libmach::Dependency;
use libmach::DependencyMap;
use libmach::DependencyOptions;
use libmach::MutableAsset;
use libmach::Config as MachConfig;

pub async fn link_and_transform(
  config: &MachConfig,
  plugins: &mut PluginContainer,
  asset_map: &mut AssetMap,
  dependency_map: &mut DependencyMap,
  asset_graph: &mut AssetGraph,
) -> Result<(), String> {
  // Take ownership of the bundling state while we transform the files.
  // We know they cannot be used elsewhere so this is safe to
  let config_local = Arc::new(config.clone());
  let plugins_local = Arc::new(std::mem::take(plugins));
  let asset_map_local = Arc::new(Mutex::new(std::mem::take(asset_map)));
  let dependency_map_local = Arc::new(Mutex::new(std::mem::take(dependency_map)));
  let asset_graph_local = Arc::new(Mutex::new(std::mem::take(asset_graph)));
  let in_progress = Arc::new(Mutex::new(HashSet::<PathBuf>::new()));

  let mut jobs = JoinSet::new();

  jobs.spawn(transform_dependency(
    config_local.clone(),
    plugins_local.clone(),
    in_progress.clone(),
    asset_map_local.clone(),
    dependency_map_local.clone(),
    asset_graph_local.clone(),
    Dependency {
      specifier: config.entry_point.to_str().unwrap().to_string(),
      is_entry: true,
      source_path: ENTRY_ASSET.clone(),
      resolve_from: ENTRY_ASSET.clone(),
      ..Dependency::default()
    },
  ));

  while let Some(result) = jobs.join_next().await {
    let result = result.unwrap();
    let result = result?;
    if let Some(dependencies) = result {
      for dependency in dependencies {
        jobs.spawn(transform_dependency(
          config_local.clone(),
          plugins_local.clone(),
          in_progress.clone(),
          asset_map_local.clone(),
          dependency_map_local.clone(),
          asset_graph_local.clone(),
          dependency,
        ));
      }
    }
  }

  //Put the results of the transformation back into the bundle state
  *plugins = Arc::try_unwrap(plugins_local).unwrap();
  *asset_map = Arc::try_unwrap(asset_map_local).unwrap().into_inner();
  *dependency_map = Arc::try_unwrap(dependency_map_local).unwrap().into_inner();
  *asset_graph = Arc::try_unwrap(asset_graph_local).unwrap().into_inner();

  Ok(())
}

async fn transform_dependency(
  config: Arc<MachConfig>,
  plugins: Arc<PluginContainer>,
  in_progress: Arc<Mutex<HashSet<PathBuf>>>,
  asset_map: Arc<Mutex<AssetMap>>,
  dependency_map: Arc<Mutex<DependencyMap>>,
  asset_graph: Arc<Mutex<AssetGraph>>,
  dependency: Dependency,
) -> Result<Option<Vec<Dependency>>, String> {
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
  let dependency_id = dependency_map.lock().await.insert(dependency);
  asset_graph.lock().await.add_edge(
    dependency_source_path.clone(),
    (dependency_id, file_path_rel.clone()),
  );
  if asset_map.lock().await.contains_key(&file_path_rel) {
    return Ok(None);
  }
  if !in_progress.lock().await.insert(file_path_rel.clone()) {
    return Ok(None);
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

  asset_map.lock().await.insert(Asset {
    name: file_target.file_stem.clone(),
    file_path: resolve_result.file_path.clone(),
    file_path_rel: file_path_rel.clone(),
    content,
    kind: asset_kind,
    bundle_behavior: dependency_bundle_behavior,
  });
  // Transformation Done

  let mut new_dependencies = Vec::<Dependency>::new();

  // Add new items to the queue
  while let Some(dependency_options) = asset_dependencies.pop() {
    new_dependencies.push(Dependency {
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

  in_progress.lock().await.remove(&file_path_rel);
  return Ok(Some(new_dependencies));
}
