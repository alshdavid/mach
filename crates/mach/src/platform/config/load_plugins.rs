use libmach::AdapterGetPluginOptions;
use libmach::AdapterMap;
use libmach::AdapterMeta;

use libmach::MachConfig;
use libmach::Machrc;
use libmach::Transformer;

use crate::platform::config::load_dynamic_adapter;
use crate::platform::plugins::resolver_javascript::ResolverJavaScript;
use crate::platform::plugins::transformer_css::TransformerCSS;
use crate::platform::plugins::transformer_drop::TransformerDrop;
use crate::platform::plugins::transformer_html::TransformerHtml;
use crate::platform::plugins::transformer_javascript::TransformerJavaScript;

use super::PluginContainer;
use super::PluginContainerSync;

pub fn load_plugins(
  config: &MachConfig,
  machrc: &Machrc,
  adapter_map: &mut AdapterMap,
) -> Result<PluginContainerSync, String> {
  let mut plugins = PluginContainer::default();
  let base_path = machrc.file_path.parent().unwrap();

  println!("  Plugins:");
  println!("    Resolvers:");

  if let Some(resolvers) = &machrc.resolvers {
    for plugin_string in resolvers {
      let Some((engine, specifier)) = plugin_string.split_once(':') else {
        return Err(format!(
          "Unable to parse engine:specifier for {}",
          plugin_string
        ));
      };

      println!("      {}:{}", engine, specifier);

      if engine == "mach" && specifier == "resolver" {
        plugins.resolvers.push(Box::new(ResolverJavaScript {}));
        continue;
      }

      if engine == "mach" {
        return Err(format!("Unable to find plugin: {}", plugin_string));
      }

      if !adapter_map.contains_key(engine) {
        adapter_map.insert(engine.to_string(), load_dynamic_adapter(&config, &engine)?);
      }

      let Some(adapter) = adapter_map.get(engine) else {
        return Err(format!("Unable to find adapter for: \"{}\"", engine));
      };

      let adapter_plugin_options = AdapterGetPluginOptions {
        specifier: specifier.to_string().clone(),
        cwd: base_path.to_path_buf().clone(),
        meta: AdapterMeta::new(),
      };
      plugins
        .resolvers
        .push(adapter.get_resolver(adapter_plugin_options)?);
    }
  }

  println!("    Transformers:");

  if let Some(transformers) = &machrc.transformers {
    for (pattern, specifiers) in transformers {
      let mut transformers = Vec::<Box<dyn Transformer>>::new();

      for plugin_string in specifiers {
        let Some((engine, specifier)) = plugin_string.split_once(':') else {
          return Err(format!(
            "Unable to parse engine:specifier for {}",
            plugin_string
          ));
        };
        println!("      {}:{:<25} {}", engine, specifier, pattern);
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

        if engine == "mach" && specifier == "transformer/drop" {
          transformers.push(Box::new(TransformerDrop {}));
          continue;
        }

        if engine == "mach" {
          return Err(format!("Unable to find plugin: \"{}\"", plugin_string));
        }

        if !adapter_map.contains_key(engine) {
          adapter_map.insert(engine.to_string(), load_dynamic_adapter(&config, &engine)?);
        }

        let Some(adapter) = adapter_map.get(engine) else {
          return Err(format!("Unable to find adapter for: {}", engine));
        };

        let adapter_plugin_options = AdapterGetPluginOptions {
          specifier: specifier.to_string().clone(),
          cwd: base_path.to_path_buf().clone(),
          meta: AdapterMeta::new(),
        };
        transformers.push(adapter.get_transformer(adapter_plugin_options)?);
      }

      plugins
        .transformers
        .transformers
        .insert(pattern.clone(), transformers);
    }
  }
  return Ok(plugins.to_sync());
}
