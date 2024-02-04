use std::sync::Arc;

use swc_core::common::SourceMap;

use crate::default_plugins::resolver::DefaultResolver;
use crate::public;
use crate::public::AssetMap;
use crate::public::Dependency;
use crate::public::DependencyMap;
use crate::public::ResolveResult;
use crate::public::Resolver;
use crate::public::SpecifierType;

pub fn transform(
  config: &public::Config,
  asset_map: &mut AssetMap,
  dependency_map: &mut DependencyMap,
  source_map: Arc<SourceMap>,
) -> Result<(), String> {
  let mut queue = Vec::<ResolveResult>::new();

  let mut resolvers = Vec::<Box<dyn Resolver>>::new();
  resolvers.push(Box::new(DefaultResolver{}));

  for resolver in &resolvers {
    let result = resolver.resolve(&Dependency {
      id: "".to_string(),
      specifier: config.entry_point.to_str().unwrap().to_string(),
      specifier_type: SpecifierType::ESM,
      is_entry: true,
      source_asset_id: "".to_string(),
      source_path: config.project_root.clone(),
      resolve_from: config.project_root.clone(),
      imported_symbols: Vec::new(),
    });
    let result = match result {
        Ok(result) => result,
        Err(err) => {
          return Err(err);
        },
    };
    let Some(result) = result else {
      return Ok(());
    };
    queue.push(result);
  }

  while let Some(resolve_result) = queue.pop() {

  }

  Ok(())
}

fn run_resolvers() -> Result<Option<ResolveResult>, String> {}