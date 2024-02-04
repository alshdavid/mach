use std::sync::Arc;

use swc_core::common::SourceMap;

use crate::default_plugins::resolver;
use crate::default_plugins::resolver::DefaultResolver;
use crate::default_plugins::transformers;
use crate::public;
use crate::public::AssetMap;
use crate::public::Dependency;
use crate::public::DependencyMap;
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
  let mut queue = Vec::<ResolveResult>::new();

  let mut transformers = Vec::<Box<dyn Transformer>>::new();

  let mut resolvers = Vec::<Box<dyn Resolver>>::new();
  resolvers.push(Box::new(DefaultResolver{}));

  let entry_result = run_resolvers(
    &resolvers,
    &Dependency {
    id: "".to_string(),
    specifier: config.entry_point.to_str().unwrap().to_string(),
    specifier_type: SpecifierType::ESM,
    is_entry: true,
    source_asset_id: "".to_string(),
    source_path: config.project_root.clone(),
    resolve_from: config.project_root.clone(),
    imported_symbols: Vec::new(),
  });

  let entry_result = match entry_result {
    Ok(e) => e,
    Err(e) => return Err(e),
  };

  let Some(result) = entry_result else {
    return Ok(());
  };

  queue.push(result);

  while let Some(resolve_result) = queue.pop() {
    println!("{:?}", resolve_result);
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