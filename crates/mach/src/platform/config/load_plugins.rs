use std::path::Path;

use normalize_path::NormalizePath;

use crate::platform::plugins::transformer_drop::TransformerDrop;
use libmach::Machrc;
use libmach::Transformer;

use super::PluginContainer;
use crate::platform::plugins::resolver::DefaultResolver;
use crate::platform::plugins::transformer_css::TransformerCSS;
use crate::platform::plugins::transformer_html::TransformerHtml;
use crate::platform::plugins::transformer_javascript::TransformerJavaScript;
use crate::platform::plugins::transformer_noop::TransformerNoop;

pub fn load_plugins(machrc: &Machrc) -> Result<PluginContainer, String> {
  let mut plugins = PluginContainer::default();
  let base_path = machrc.file_path.parent().unwrap();

  if let Some(resolvers) = &machrc.resolvers {
    for plugin_string in resolvers {
      let (engine, specifier) = parse_plugin_string(&base_path, plugin_string)?;

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
        let (engine, specifier) = parse_plugin_string(&base_path, plugin_string)?;

        if engine == "mach" && specifier == "transformer/javascript" {
          transformers.push(Box::new(TransformerJavaScript {}));
          continue;
        }

        if engine == "mach" && specifier == "transformer/css" {
          transformers.push(Box::new(TransformerCSS {}));
          continue;
        }

        if engine == "mach" && specifier == "transformer/html" {
          transformers.push(Box::new(TransformerHtml {}));
          continue;
        }

        if engine == "mach" && specifier == "transformer/noop" {
          transformers.push(Box::new(TransformerNoop {}));
          continue;
        }

        if engine == "mach" && specifier == "transformer/drop" {
          transformers.push(Box::new(TransformerDrop {}));
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

fn parse_plugin_string(
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

  // if engine == "node" {
  //   if let Ok(specifier) = native_node_resolve(&base_path, &specifier).await {
  //     return Ok((engine.to_string(), specifier));
  //   };
  // }

  return Err("Could not load plugin string".to_string());
}
