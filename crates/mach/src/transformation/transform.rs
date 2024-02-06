use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use crate::default_plugins::resolver::DefaultResolver;
use crate::default_plugins::transformers::javascript::DefaultJSTransformer;
use crate::public;
use crate::public::Asset;
use crate::public::AssetGraph;
use crate::public::AssetId;
use crate::public::AssetMap;
use crate::public::Dependency;
use crate::public::DependencyGraph;
use crate::public::DependencyOptions;
use crate::public::MutableAsset;
use crate::public::ResolveResult;
use crate::public::Resolver;
use crate::public::SpecifierType;
use crate::public::Transformer;

pub fn transform(
  config: &public::Config,
  asset_map: &mut AssetMap,
  asset_graph: &mut AssetGraph,
  dependency_graph: &mut DependencyGraph,
) -> Result<(), String> {
  let mut done_assets = HashSet::<PathBuf>::new();
  let mut asset_index = HashMap::<PathBuf, AssetId>::new();

  let mut queue = Vec::<Dependency>::new();
  queue.push(Dependency {
    id:  Dependency::generate_id(
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

  // Load transformer plugins
  let mut transformers = Vec::<Box<dyn Transformer>>::new();
  transformers.push(Box::new(DefaultJSTransformer{}));

  // Load resolver plugins
  let mut resolvers = Vec::<Box<dyn Resolver>>::new();
  resolvers.push(Box::new(DefaultResolver{}));

  while let Some(dependency) = queue.pop() {
    // Run Resolvers
    let mut resolve_result: Option<ResolveResult> = None;

    for resolver in &resolvers {
      let result = resolver.resolve(&dependency);
      let result = match result {
          Ok(result) => result,
          Err(err) => {
            return Err(err);
          },
      };
      let Some(result) = result else {
        break;
      };
      resolve_result = Some(result);
      break;
    }

    let Some(resolve_result) = resolve_result else {
      continue;
    };

    let parent_asset_id = {
      if let Some(v) = asset_map.get(&dependency.source_asset_id) {
        v.id.clone()
      } else {
        "".to_string()
      }
    };
    dependency_graph.insert(dependency);

    if let Some(target_asset) = asset_map.get_file(&resolve_result.file_path) {
      asset_graph.add_edge(target_asset.id.clone(), parent_asset_id.clone());
      continue;
    }

    let Ok(mut code) = fs::read_to_string(&resolve_result.file_path) else {
      return Err("Unable to read file".to_string());
    };

    let new_asset_id = Asset::generate_id(
      &config.project_root,
      &resolve_result.file_path,
      &code,
    );
    asset_graph.add_edge(new_asset_id.clone(), parent_asset_id.clone());


    let mut dependencies = Vec::<DependencyOptions>::new();

    let mut mutable_asset = MutableAsset::new( 
      resolve_result.file_path.clone(),
      &mut code,
      &mut dependencies,
    );

    for transformer in &transformers {
      if let Err(msg) = transformer.transform(&mut mutable_asset, &config) {
        return Err(msg);
      }
    }
    
    let new_asset_id = asset_map.insert(Asset{
        id: new_asset_id,
        file_path: resolve_result.file_path.clone(),
        code,
    });

    while let Some(dependency_options) = dependencies.pop() {
      queue.push(Dependency{
        id: Dependency::generate_id(
          &new_asset_id, 
          &dependency_options.specifier,
        ),
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

  Ok(())
}
