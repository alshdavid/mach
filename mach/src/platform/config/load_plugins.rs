use libmach::AdapterMap;
use libmach::AdapterOptions;
use libmach::Machrc;
use libmach::Transformer;
use libmach::AdapterOption;

use crate::platform::plugins::resolver_javascript::ResolverJavaScript;
use crate::platform::plugins::transformer_css::TransformerCSS;
use crate::platform::plugins::transformer_html::TransformerHtml;
use crate::platform::plugins::transformer_javascript::TransformerJavaScript;
use crate::platform::plugins::transformer_noop::DefaultTransformerNoop;
use super::load_dynamic_adapter;
use super::PluginContainer;

pub async fn load_plugins(
  machrc: &Machrc,
  adapter_map: &mut AdapterMap,
) -> Result<PluginContainer, String> {
  let mut plugins = PluginContainer::default();
  let base_path = machrc.file_path.parent().unwrap();

  if let Some(resolvers) = &machrc.resolvers {
    for plugin_string in resolvers {
      let Some((engine, specifier)) = plugin_string.split_once(':') else {
        return Err(format!("Unable to parse engine:specifier for {}", plugin_string));
      };
      
      println!("resolver - {}:{}", engine, specifier);
      if engine == "mach" && specifier == "resolver" {
        plugins.resolvers.push(Box::new(ResolverJavaScript {}));
        continue;
      }

      if engine == "mach" {
        continue;
      }

      if !adapter_map.contains_key(engine) {
        adapter_map.insert(engine.to_string(), load_dynamic_adapter(&engine).await?); 
      }

      let Some(adapter) = adapter_map.get(engine) else {
        return Err(format!("Unable to find adapter for: {}", engine));
      };

      let mut adapter_options = AdapterOptions::default();
      adapter_options.insert("specifier".to_string(), AdapterOption::String(specifier.to_string()));
      adapter_options.insert("cwd".to_string(), AdapterOption::PathBuf(base_path.to_path_buf()));
      plugins.resolvers.push(adapter.get_resolver(adapter_options).await?);
    }
  }

  if let Some(transformers) = &machrc.transformers {
    for (pattern, specifiers) in transformers {
      let mut transformers = Vec::<Box<dyn Transformer>>::new();

      for plugin_string in specifiers {
        let Some((engine, specifier)) = plugin_string.split_once(':') else {
          return Err(format!("Unable to parse engine:specifier for {}", plugin_string));
        };
        println!("resolver - {}:{}", engine, specifier);


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
          transformers.push(Box::new(DefaultTransformerNoop {}));
          continue;
        }

        if engine == "mach" {
          continue;
        }

        if !adapter_map.contains_key(engine) {
          adapter_map.insert(engine.to_string(), load_dynamic_adapter(&engine).await?); 
        }
  
        let Some(adapter) = adapter_map.get(engine) else {
          return Err(format!("Unable to find adapter for: {}", engine));
        };
  
        let mut adapter_options = AdapterOptions::default();
        adapter_options.insert("specifier".to_string(), AdapterOption::String(specifier.to_string()));
        adapter_options.insert("cwd".to_string(), AdapterOption::PathBuf(base_path.to_path_buf()));
        transformers.push(adapter.get_transformer(adapter_options).await?);
      }

      plugins
        .transformers
        .transformers
        .insert(pattern.clone(), transformers);
    }
  }
  return Ok(plugins);
}
