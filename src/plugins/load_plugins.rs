use std::path::Path;
use std::sync::Arc;

use normalize_path::NormalizePath;

use crate::adapters::node_js::native_node_resolve;
use crate::adapters::node_js::NodeAdapter;
use crate::public::Machrc;
use crate::public::Transformer;

use super::builtin::resolver::DefaultResolver;
use super::builtin::resolver_node_js::ResolverNodeJs;
use super::builtin::transformer_javascript::DefaultTransformerJs;
use super::builtin::transformer_node_js::TransformerNodeJs;
use super::PluginContainer;

pub async fn load_plugins(
  machrc: &Machrc,
  node_adapter: Arc<NodeAdapter>,
) -> Result<PluginContainer, ()> {
  let mut plugins = PluginContainer::default();
  let base_path = machrc.file_path.parent().unwrap();

  if let Some(resolvers) = &machrc.resolvers {
    for plugin_string in resolvers {
      let Ok((engine, specifier)) = parse_plugin_string(&base_path, plugin_string).await else {
        return Err(());
      };

      if engine == "mach" && specifier == "resolver" {
        plugins.resolvers.push(Box::new(DefaultResolver {}));
        continue;
      }

      if engine == "node" {
        let plugin = ResolverNodeJs::new(node_adapter.clone(), &specifier).await;
        plugins.resolvers.push(Box::new(plugin));
        continue;
      }
    }
  }

  if let Some(transformers) = &machrc.transformers {
    for (pattern, specifiers) in transformers {
      let mut transformers = Vec::<Box<dyn Transformer>>::new();

      for plugin_string in specifiers {
        let Ok((engine, specifier)) = parse_plugin_string(&base_path, plugin_string).await else {
          return Err(());
        };

        if engine == "mach" && specifier == "transformers/javascript" {
          transformers.push(Box::new(DefaultTransformerJs {}));
          continue;
        }

        if engine == "node" {
          let plugin = TransformerNodeJs::new(node_adapter.clone(), &specifier).await;
          transformers.push(Box::new(plugin));
          continue;
        }
      }

      plugins
        .transformers
        .transformers
        .insert(pattern.clone(), transformers);
    }
  }
  return Ok(plugins);
}

async fn parse_plugin_string(
  base_path: &Path,
  plugin_string: &str,
) -> Result<(String, String), ()> {
  let (engine, specifier) = plugin_string.split_once(':').unwrap();

  let engine = engine.to_string();
  let specifier = specifier.to_string();

  if engine == "mach" {
    return Ok((engine, specifier));
  }

  if specifier.starts_with('.') {
    let new_path = base_path.join(specifier).normalize();
    let specifier = new_path.to_str().unwrap().to_string();
    return Ok((engine, specifier));
  }

  if specifier.starts_with('/') || specifier.starts_with('\\') {
    return Ok((engine, specifier));
  }

  if engine == "node" {
    if let Ok(specifier) = native_node_resolve(&base_path, &specifier).await {
      return Ok((engine.to_string(), specifier));
    };
  }

  return Err(());
}
