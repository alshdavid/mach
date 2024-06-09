use std::fmt::Debug;
use std::sync::Arc;

use anyhow;

use crate::plugins::resolver_javascript::resolve;
use crate::public::AdapterOutgoingRequest;
use crate::public::AdapterOutgoingRequestTransformerLoadConfig;
use crate::public::AdapterOutgoingRequestTransformerRegister;
use crate::public::AdapterOutgoingRequestTransformerTransform;
use crate::public::AdapterOutgoingResponse;
use crate::public::DependencyOptions;
use crate::public::MachConfig;
use crate::public::MutableAsset;
use crate::public::RpcHost;
use crate::public::Transformer;

pub struct TransformerAdapter {
  transformer_specifier: String,
  adapter: Arc<dyn RpcHost>,
}

impl TransformerAdapter {
  pub fn new(
    config: &MachConfig,
    initial_specifier: &str,
    adapter: Arc<dyn RpcHost>,
  ) -> anyhow::Result<Self> {
    // let specifier = resolve(&config.project_root, initial_specifier)?;
    // if !specifier.exists() {
    //   return Err(format!(
    //     "Plugin not found for specifier: {:?}",
    //     initial_specifier
    //   ));
    // }
    // let specifier = specifier.to_str().unwrap().to_string();

    // adapter.send_all(AdapterOutgoingRequest::TransformerRegister(
    //   AdapterOutgoingRequestTransformerRegister {
    //     specifier: specifier.clone(),
    //   },
    // ))?;

    // adapter.send_all(AdapterOutgoingRequest::TransformerLoadConfig(
    //   AdapterOutgoingRequestTransformerLoadConfig {
    //     specifier: specifier.clone(),
    //   },
    // ))?;

    // Ok(Self {
    //   transformer_specifier: specifier.to_string(),
    //   adapter,
    // })
    todo!()
  }
}

impl Transformer for TransformerAdapter {
  fn transform(
    &self,
    asset: &mut MutableAsset,
    _config: &MachConfig,
  ) -> Result<(), String> {
    //     let response = self
    //       .adapter
    //       .send_and_wait(AdapterOutgoingRequest::TransformerTransform(
    //         AdapterOutgoingRequestTransformerTransform {
    //           specifier: self.transformer_specifier.clone(),
    //           file_path: asset.file_path.to_path_buf(),
    //           kind: asset.kind.clone(),
    //           content: asset.get_bytes().to_vec(),
    //         },
    //       ))?;

    //     if let AdapterOutgoingResponse::Err(err) = response {
    //       return Err(err);
    //     };

    //     let AdapterOutgoingResponse::TransformerTransform(response) = response else {
    //       panic!();
    //     };

    //     asset.set_bytes(response.content);
    //     *asset.kind = response.kind;

    //     for dependency in response.dependencies {
    //       asset.add_dependency(DependencyOptions {
    //         specifier: dependency.specifier,
    //         specifier_type: dependency.specifier_type,
    //         priority: dependency.priority,
    //         resolve_from: dependency.resolve_from,
    //         imported_symbols: dependency.imported_symbols,
    //         reimported_symbols: Default::default(),
    //         bundle_behavior: dependency.bundle_behavior,
    //       })
    //     }

    //     Ok(())
    //   }
    // }
    todo!()
  }
}

impl Debug for TransformerAdapter {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.write_str(&format!(
      "TransformerNodeAdapter({})",
      self.transformer_specifier
    ))
  }
}
