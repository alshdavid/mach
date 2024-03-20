use std::path::Path;
use std::sync::Arc;

use normalize_path::NormalizePath;

use crate::platform::adapters::node_js::native_node_resolve;
use crate::platform::adapters::node_js::NodeAdapter;
use crate::public::Machrc;
use crate::public::Transformer;

use crate::platform::plugins::resolver::DefaultResolver;
use crate::platform::plugins::transformer_css::DefaultTransformerCSS;
use crate::platform::plugins::transformer_html::DefaultTransformerHtml;
use crate::platform::plugins::transformer_javascript::DefaultTransformerJavaScript;
use crate::platform::plugins::transformer_noop::DefaultTransformerNoop;
use super::PluginContainer;

pub async fn load_plugins(
  machrc: &Machrc,
  _node_adapter: Arc<NodeAdapter>,
) -> Result<PluginContainer, String> {
  let mut plugins = PluginContainer::default();
  let base_path = machrc.file_path.parent().unwrap();

  if let Some(resolvers) = &machrc.resolvers {
    for plugin_string in resolvers {
      let (engine, specifier) = parse_plugin_string(&base_path, plugin_string).await?;

      if engine == "mach" && specifier == "resolver" {
        plugins.resolvers.push(Box::new(DefaultResolver {}));
        continue;
      }

      // if engine == "node" {
      //   let plugin = ResolverNodeJs::new(node_adapter.clone(), &specifier).await;
      //   plugins.resolvers.push(Box::new(plugin));
      //   continue;
      // }
    }
  }

  if let Some(transformers) = &machrc.transformers {
    for (pattern, specifiers) in transformers {
      let mut transformers = Vec::<Box<dyn Transformer>>::new();

      for plugin_string in specifiers {
        let (engine, specifier) = parse_plugin_string(&base_path, plugin_string).await?;

        if engine == "mach" && specifier == "transformer/javascript" {
          transformers.push(Box::new(DefaultTransformerJavaScript {}));
          continue;
        }

        if engine == "mach" && specifier == "transformer/css" {
          transformers.push(Box::new(DefaultTransformerCSS {}));
          continue;
        }

        if engine == "mach" && specifier == "transformer/html" {
          transformers.push(Box::new(DefaultTransformerHtml {}));
          continue;
        }

        if engine == "mach" && specifier == "transformer/noop" {
          transformers.push(Box::new(DefaultTransformerNoop {}));
          continue;
        }

        // if engine == "node" {
        //   let plugin = TransformerNodeJs::new(node_adapter.clone(), &specifier).await;
        //   transformers.push(Box::new(plugin));
        //   continue;
        // }
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
) -> Result<(String, String), String> {
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

  return Err("Could not load plugin string".to_string());
}
