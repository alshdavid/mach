use std::fs;
use std::path::PathBuf;

use crate::plugins::PluginContainer;
use crate::public;
use crate::public::Asset;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Dependency;
use crate::public::DependencyGraph;
use crate::public::DependencyOptions;
use crate::public::MutableAsset;
use crate::public::ResolveResult;
use crate::public::SpecifierType;

pub async fn transform(
  config: &public::Config,
  asset_map: &mut AssetMap,
  asset_graph: &mut AssetGraph,
  dependency_graph: &mut DependencyGraph,
  plugins: &PluginContainer,
) -> Result<(), String> {
  let mut queue = Vec::<Dependency>::new();
  queue.push(Dependency {
    id: Dependency::generate_id(
      &"".into(),
      &config.entry_point.to_str().unwrap().to_string(),
    ),
    specifier: config.entry_point.to_str().unwrap().to_string(),
    specifier_type: SpecifierType::ESM,
    is_entry: true,
    source_asset_id: "".to_string(),
    source_path: PathBuf::new(),
    resolve_from: PathBuf::new(),
    imported_symbols: Vec::new(),
  });

  let mut skipped = 0;

  while let Some(dependency) = queue.pop() {
    // Resolve Start
    let mut resolve_result: Option<ResolveResult> = None;

    for resolver in &plugins.resolvers {
      let result = resolver.resolve(&dependency).await;
      let result = match result {
        Ok(result) => result,
        Err(err) => {
          // println!("{}", err);
          skipped += 1;
          continue;
          // return Err(err);
        }
      };
      resolve_result = result;
      if resolve_result.is_some() {
        break;
      }
    }

    let Some(resolve_result) = resolve_result else {
      skipped += 1;
      continue;
    };
    // Resolve Done

    // Dependency Graph
    let parent_asset_id = dependency.source_asset_id.clone();

    dependency_graph.insert(dependency);

    if let Some(target_asset) = asset_map.get_file(&resolve_result.file_path) {
      asset_graph.add_edge(parent_asset_id.clone(), target_asset.id.clone());
      continue;
    }

    let new_asset_id = Asset::generate_id(&config.project_root, &resolve_result.file_path);
    asset_graph.add_edge(parent_asset_id.clone(), new_asset_id.clone());
    // Dependency Graph Done

    // Transformation
    let Ok(mut content) = fs::read(&resolve_result.file_path) else {
      return Err(format!("Unable to read file: {:?}", resolve_result.file_path));
    };

    let mut dependencies = Vec::<DependencyOptions>::new();

    let mut mutable_asset = MutableAsset::new(
      resolve_result.file_path.clone(),
      &mut content,
      &mut dependencies,
    );

    let Some(transformers) = plugins.transformers.get(&resolve_result.file_path) else {
      skipped += 1;
      // println!("No transformer found {:?}", resolve_result.file_path);
      continue;
    };

    for transformer in transformers {
      if let Err(msg) = transformer.transform(&mut mutable_asset, &config).await {
        return Err(msg);
      }
    }

    let new_asset_id = asset_map.insert(Asset {
      id: new_asset_id,
      file_path: resolve_result.file_path.clone(),
      content,
    });
    // Transformation Done

    // Add new items to the queue
    while let Some(dependency_options) = dependencies.pop() {
      queue.push(Dependency {
        id: Dependency::generate_id(&new_asset_id, &dependency_options.specifier),
        specifier: dependency_options.specifier.clone(),
        specifier_type: dependency_options.specifier_type,
        is_entry: false,
        source_asset_id: new_asset_id.clone(),
        source_path: resolve_result.file_path.clone(),
        resolve_from: resolve_result.file_path.clone(),
        imported_symbols: vec![],
      });
    }
  }

  println!("Skipped: {}", skipped);


  Ok(())
}
