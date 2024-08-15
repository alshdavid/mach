use std::sync::Arc;

use anyhow;

use crate::plugins::resolver_javascript::ResolverJavaScript;
use crate::plugins::resolver_rpc::ResolverAdapter;
use crate::plugins::transformer_css::TransformerCSS;
use crate::plugins::transformer_drop::TransformerDrop;
use crate::plugins::transformer_html::TransformerHtml;
use crate::plugins::transformer_javascript::TransformerJavaScript;
use crate::plugins::transformer_json::TransformerJson;
use crate::plugins::transformer_rpc::TransformerAdapter;
use crate::types::Compilation;
use crate::types::Transformer;

pub fn load_plugins(c: &mut Compilation) -> anyhow::Result<()> {
  if let Some(resolvers) = &c.machrc.resolvers {
    for plugin_string in resolvers {
      match plugin_string.as_str() {
        // Built-in
        "mach:resolver" => {
          c.plugins
            .resolvers
            .push(Arc::new(ResolverJavaScript::new()));
          continue;
        }
        // External
        specifier => {
          let Some((engine, specifier)) = specifier.split_once(':') else {
            return Err(anyhow::anyhow!(format!(
              "Unable to parse engine:specifier for {}",
              plugin_string
            )));
          };

          let Some(adapter) = c.rpc_hosts.get(engine) else {
            return Err(anyhow::anyhow!(format!(
              "No plugin runtime for engine: {}\nCannot load plugin: {}",
              engine, specifier
            )));
          };

          c.plugins.resolvers.push(Arc::new(ResolverAdapter::new(
            &c.config,
            specifier,
            adapter.clone(),
          )?));
        }
      }
    }
  }

  if let Some(transformers) = &c.machrc.transformers {
    for (pattern, specifiers) in transformers {
      let mut transformers = Vec::<Arc<dyn Transformer>>::new();

      for plugin_string in specifiers {
        let Some((engine, specifier)) = plugin_string.split_once(':') else {
          return Err(anyhow::anyhow!(format!(
            "Unable to parse engine:specifier for {}",
            plugin_string
          )));
        };
        if engine == "mach" && specifier == "transformer/javascript" {
          transformers.push(Arc::new(TransformerJavaScript {}));
          continue;
        }

        if engine == "mach" && specifier == "transformer/css" {
          transformers.push(Arc::new(TransformerCSS {}));
          continue;
        }

        if engine == "mach" && specifier == "transformer/html" {
          transformers.push(Arc::new(TransformerHtml {}));
          continue;
        }

        if engine == "mach" && specifier == "transformer/json" {
          transformers.push(Arc::new(TransformerJson {}));
          continue;
        }

        if engine == "mach" && specifier == "transformer/drop" {
          transformers.push(Arc::new(TransformerDrop {}));
          continue;
        }

        let Some(adapter) = c.rpc_hosts.get(engine) else {
          return Err(anyhow::anyhow!(format!(
            "No plugin runtime for engine: {}\nCannot load plugin: {}",
            engine, specifier
          )));
        };

        adapter.start()?;

        transformers.push(Arc::new(TransformerAdapter::new(
          &c.config,
          specifier,
          adapter.clone(),
        )?));
      }

      c.plugins
        .transformers
        .transformers
        .insert(pattern.clone(), transformers);
    }
  }

  return Ok(());
}
