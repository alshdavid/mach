use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use swc_core::common::SourceMap;

use crate::default_plugins::resolver::DefaultResolver;
use crate::default_plugins::transformers::javascript::DefaultJSTransformer;
use crate::public;
use crate::public::Asset;
use crate::public::AssetMap;
use crate::public::Dependency;
use crate::public::DependencyKind;
use crate::public::DependencyLegacy;
use crate::public::DependencyMap;
use crate::public::DependencyOptions;
use crate::public::MutableAsset;
use crate::public::ResolveResult;
use crate::public::Resolver;
use crate::public::SpecifierType;
use crate::public::Transformer;

pub fn transform(
  config: &public::Config,
  asset_map: &mut AssetMap,
  dependency_map: &mut DependencyMap,
  source_map: Arc<SourceMap>,
) -> Result<(), String> {
  let mut done_files = HashSet::<PathBuf>::new();
  let mut queue = Vec::<Dependency>::new();
  queue.push(Dependency {
    id: "".to_string(),
    specifier: config.entry_point.to_str().unwrap().to_string(),
    specifier_type: SpecifierType::ESM,
    is_entry: true,
    source_asset_id: "".to_string(),
    source_path: config.project_root.clone(),
    resolve_from: config.project_root.clone(),
    imported_symbols: Vec::new(),
  });

  let mut transformers = Vec::<Box<dyn Transformer>>::new();
  transformers.push(Box::new(DefaultJSTransformer{}));

  let mut resolvers = Vec::<Box<dyn Resolver>>::new();
  resolvers.push(Box::new(DefaultResolver{}));

  while let Some(dependency) = queue.pop() {
    let entry_result = run_resolvers(
      &resolvers,
      &dependency,
    );

    let entry_result = match entry_result {
      Ok(e) => e,
      Err(e) => return Err(e),
    };

    let Some(result) = entry_result else {
      continue;
    };

    dependency_map.insert(&dependency.source_asset_id.clone(), DependencyLegacy{
      parent_asset_id: dependency.source_asset_id.clone(),
      target_asset_id: result.file_path.to_str().unwrap().to_string(),//clone(),
      import_specifier: dependency.specifier.clone(),
      kind: DependencyKind::Static,
    });

    if !done_files.insert(result.file_path.clone()) {
      continue;
    }

    let Ok(mut code) = fs::read_to_string(&result.file_path) else {
      return Err("Unable to read file".to_string());
    };

    let mut dependencies = Vec::<DependencyOptions>::new();

    let mut mutable_asset = MutableAsset::new( 
      result.file_path.clone(),
      &mut code,
      &mut dependencies,
    );

    for transformer in &transformers {
      if let Err(msg) = transformer.transform(&mut mutable_asset, &config) {
        return Err(msg);
      }
    }

    let new_asset_id = asset_map.insert(Asset::new(
      &config.project_root,
      &result.file_path,
      code,
    ));

    while let Some(dependency_options) = dependencies.pop() {
      queue.push(Dependency{
        id: "".to_string(),
        specifier: dependency_options.specifier.clone(),
        specifier_type: dependency_options.specifier_type,
        is_entry: false,
        source_asset_id: new_asset_id.clone(),
        source_path: result.file_path.clone(),
        resolve_from: result.file_path.clone(),
        imported_symbols: vec![],
      });
    }
  }

  Ok(())
}

fn run_resolvers(
  resolvers: &Vec<Box<dyn Resolver>>,
  dependency: &Dependency,
) -> Result<Option<ResolveResult>, String> {
  for resolver in resolvers {
    let result = resolver.resolve(dependency);
    let result = match result {
        Ok(result) => result,
        Err(err) => {
          return Err(err);
        },
    };
    let Some(result) = result else {
      return Ok(None);
    };
    return Ok(Some(result))
  }
  return Err("Unable to resolve".to_string());
}