use std::fmt::Debug;

use crate::platform::adapters::nodejs::NodejsAdapter;
use crate::platform::plugins::resolver_javascript::resolve;
use crate::public::nodejs::client::NodejsClientRequest;
use crate::public::nodejs::client::NodejsClientRequestTransformerLoadConfig;
use crate::public::nodejs::client::NodejsClientRequestTransformerRegister;
use crate::public::nodejs::client::NodejsClientRequestTransformerTransform;
use crate::public::nodejs::client::NodejsClientResponse;
use crate::public::DependencyOptions;
use crate::public::MachConfig;
use crate::public::MutableAsset;
use crate::public::Transformer;

pub struct TransformerNodeJs {
  transformer_specifier: String,
  nodejs_adapter: NodejsAdapter,
}

impl TransformerNodeJs {
  pub fn new(
    config: &MachConfig,
    initial_specifier: &str,
    nodejs_adapter: NodejsAdapter,
  ) -> Result<Self, String> {
    let specifier = resolve(&config.project_root, initial_specifier)?;
    if !specifier.exists() {
      return Err(format!(
        "Plugin not found for specifier: {:?}",
        initial_specifier
      ));
    }
    let specifier = specifier.to_str().unwrap().to_string();

    nodejs_adapter.send_all(NodejsClientRequest::TransformerRegister(
      NodejsClientRequestTransformerRegister {
        specifier: specifier.clone(),
      },
    ))?;

    nodejs_adapter.send_all(NodejsClientRequest::TransformerLoadConfig(
      NodejsClientRequestTransformerLoadConfig {
        specifier: specifier.clone(),
      },
    ))?;

    Ok(Self {
      transformer_specifier: specifier.to_string(),
      nodejs_adapter,
    })
  }
}

impl Transformer for TransformerNodeJs {
  fn transform(
    &self,
    asset: &mut MutableAsset,
    _config: &MachConfig,
  ) -> Result<(), String> {
    let response = self
      .nodejs_adapter
      .send_and_wait(NodejsClientRequest::TransformerTransform(
        NodejsClientRequestTransformerTransform {
          specifier: self.transformer_specifier.clone(),
          file_path: asset.file_path.to_path_buf(),
          kind: asset.kind.clone(),
          content: asset.get_bytes().to_vec(),
        },
      ));

    if let NodejsClientResponse::Err(err) = response {
      return Err(err);
    };

    let NodejsClientResponse::TransformerTransform(result) = response else {
      panic!();
    };

    asset.set_bytes(result.content);
    *asset.kind = result.kind;

    for dependency in result.dependencies {
      asset.add_dependency(DependencyOptions {
        specifier: dependency.specifier,
        specifier_type: dependency.specifier_type,
        priority: dependency.priority,
        resolve_from: dependency.resolve_from,
        imported_symbols: dependency.imported_symbols,
        bundle_behavior: dependency.bundle_behavior,
      })
    }

    Ok(())
  }
}

impl Debug for TransformerNodeJs {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.write_str(&format!(
      "TransformerNodeJs({})",
      self.transformer_specifier
    ))
  }
}
