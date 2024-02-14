use std::collections::HashMap;
use std::sync::Arc;

use normalize_path::NormalizePath;

use crate::adapters::node_js::native_node_resolve;
use crate::public::Machrc;
use crate::public::Resolver;
use crate::adapters::node_js::NodeAdapter;
use crate::public::Transformer;

use super::builtin::resolver::DefaultResolver;
use super::builtin::resolver_node_js::ResolverNodeJs;
use super::builtin::transformer_javascript::DefaultTransformerJs;

#[derive(Default, Debug)]
pub struct Plugins {
  pub resolvers: Vec<Box<dyn Resolver>>,
  pub transformers: HashMap<String, Vec<Box<dyn Transformer>>>,
}

pub async fn load_plugins(
  machrc: &Option<Machrc>,
  node_adapter: Arc<NodeAdapter>,
) -> Result<Plugins, ()> {
  let Some(machrc) = machrc else {
    todo!();
  };

  let mut plugins = Plugins::default();

  if let Some(resolvers) = &machrc.resolvers {
    for resolver in resolvers {
      let (engine, specifier) = resolver.split_once(':').unwrap();

      if engine == "mach" {
        plugins.resolvers.push(Box::new(DefaultResolver{}));
        continue;
      }
      if engine == "node" && specifier.starts_with('.') {
        let specifier = machrc.file_path.parent().unwrap().join(specifier).normalize();
        let specifier = specifier.to_str().unwrap();
        plugins.resolvers.push(Box::new(ResolverNodeJs::new(node_adapter.clone(), &specifier).await));
        continue;
      }
      if engine == "node" && (specifier.starts_with('/') || specifier.starts_with('\\')) {
        plugins.resolvers.push(Box::new(ResolverNodeJs::new(node_adapter.clone(), &specifier).await));
        continue;
      }
      if engine == "node" {
        let Ok(path) = native_node_resolve(&machrc.file_path.parent().unwrap(), &specifier).await else {
          return Err(());
        };
        plugins.resolvers.push(Box::new(ResolverNodeJs::new(node_adapter.clone(), &path).await));
      }
    }
  }

  if let Some(transformers) = &machrc.transformers {
    for (pattern, specifiers) in transformers {
      let mut transformers = Vec::<Box<dyn Transformer>>::new();

      for specifier in specifiers {
        let (engine, _specifier) = specifier.split_once(':').unwrap();

        if engine == "mach" {
          transformers.push(Box::new(DefaultTransformerJs{}));
          continue;
        }
      }

      plugins.transformers.insert(pattern.clone(), transformers);
    }
  }
  return Ok(plugins);
}
