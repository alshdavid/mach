use super::PluginContainer;
use super::PluginContainerSync;
use crate::plugins::resolver_rpc::ResolverAdapter;
use crate::plugins::resolver_javascript::ResolverJavaScript;
use crate::plugins::transformer_rpc::TransformerAdapter;
use crate::plugins::transformer_css::TransformerCSS;
use crate::plugins::transformer_drop::TransformerDrop;
use crate::plugins::transformer_html::TransformerHtml;
use crate::plugins::transformer_javascript::TransformerJavaScript;
use crate::plugins::transformer_json::TransformerJson;
use crate::public::RpcHosts;
use crate::public::MachConfig;
use crate::public::Machrc;
use crate::public::Transformer;

pub fn load_plugins(
  config: &MachConfig,
  machrc: &Machrc,
  adapter_map: &RpcHosts,
) -> Result<PluginContainerSync, String> {
  let mut plugins = PluginContainer::default();

  if let Some(resolvers) = &machrc.resolvers {
    for plugin_string in resolvers {
      let Some((engine, specifier)) = plugin_string.split_once(':') else {
        return Err(format!(
          "Unable to parse engine:specifier for {}",
          plugin_string
        ));
      };

      if engine == "mach" && specifier == "resolver" {
        plugins.resolvers.push(Box::new(ResolverJavaScript::new()));
        continue;
      }

      let Some(adapter) = adapter_map.get(engine) else {
        return Err(format!(
          "No plugin runtime for engine: {}\nCannot load plugin: {}",
          engine, specifier
        ));
      };

      adapter.init()?;

      plugins.resolvers.push(Box::new(ResolverAdapter::new(
        &*config,
        specifier,
        adapter.clone(),
      )?));

      continue;
    }
  }

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

        if engine == "mach" && specifier == "transformer/json" {
          transformers.push(Box::new(TransformerJson {}));
          continue;
        }

        if engine == "mach" && specifier == "transformer/drop" {
          transformers.push(Box::new(TransformerDrop {}));
          continue;
        }

        let Some(adapter) = adapter_map.get(engine) else {
          return Err(format!(
            "No plugin runtime for engine: {}\nCannot load plugin: {}",
            engine, specifier
          ));
        };

        adapter.init()?;

        transformers.push(Box::new(TransformerAdapter::new(
          &*config,
          specifier,
          adapter.clone(),
        )?));
      }

      plugins
        .transformers
        .transformers
        .insert(pattern.clone(), transformers);
    }
  }
  return Ok(plugins.to_sync());
}
