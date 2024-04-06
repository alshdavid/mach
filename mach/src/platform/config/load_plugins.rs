use crate::public::MachConfig;
use crate::public::Machrc;
use crate::public::Transformer;

use crate::platform::plugins::resolver_javascript::ResolverJavaScript;
use crate::platform::plugins::transformer_css::TransformerCSS;
use crate::platform::plugins::transformer_drop::TransformerDrop;
use crate::platform::plugins::transformer_html::TransformerHtml;
use crate::platform::plugins::transformer_javascript::TransformerJavaScript;

use super::PluginContainer;
use super::PluginContainerSync;

pub fn load_plugins(
  _config: &MachConfig,
  machrc: &Machrc,
) -> Result<PluginContainerSync, String> {
  let mut plugins = PluginContainer::default();

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

      return Err(format!(
        "Unable to find plugin: {}:{}",
        engine, plugin_string
      ));
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