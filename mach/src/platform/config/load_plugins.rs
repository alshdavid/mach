use super::PluginContainer;
use super::PluginContainerSync;
use crate::platform::adapters::nodejs::NodejsAdapter;
use crate::platform::plugins::resolver_javascript::ResolverJavaScript;
use crate::platform::plugins::resolver_nodejs::ResolverNodejs;
use crate::platform::plugins::transformer_css::TransformerCSS;
use crate::platform::plugins::transformer_drop::TransformerDrop;
use crate::platform::plugins::transformer_html::TransformerHtml;
use crate::platform::plugins::transformer_javascript::TransformerJavaScript;
use crate::platform::plugins::transformer_nodejs::TransformerNodeJs;
use crate::public::MachConfig;
use crate::public::Machrc;
use crate::public::Transformer;

pub fn load_plugins(
  config: &MachConfig,
  machrc: &Machrc,
  nodejs_adapter: NodejsAdapter,
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

      if engine == "node" {
        nodejs_adapter.start_nodejs()?;
        plugins.resolvers.push(Box::new(ResolverNodejs::new(
          &*config,
          specifier,
          nodejs_adapter.clone(),
        )?));
        continue;
      }

      return Err(format!(
        "Unable to find plugin: {}:{}",
        engine, plugin_string
      ));
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

        if engine == "mach" && specifier == "transformer/drop" {
          transformers.push(Box::new(TransformerDrop {}));
          continue;
        }

        if engine == "node" {
          nodejs_adapter.start_nodejs()?;
          transformers.push(Box::new(TransformerNodeJs::new(
            &config,
            specifier,
            nodejs_adapter.clone(),
          )?));
          continue;
        }

        return Err(format!(
          "Unable to find plugin: {}:{}",
          engine, plugin_string
        ));
      }

      plugins
        .transformers
        .transformers
        .insert(pattern.clone(), transformers);
    }
  }
  return Ok(plugins.to_sync());
}
