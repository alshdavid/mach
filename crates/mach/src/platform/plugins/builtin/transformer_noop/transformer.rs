use async_trait::async_trait;

use crate::platform::public::Config;
use crate::platform::public::DependencyOptions;
use crate::platform::public::MutableAsset;
use crate::platform::public::Transformer;

#[derive(Debug)]
pub struct DefaultTransformerNoop {}

#[async_trait]
impl Transformer for DefaultTransformerNoop {
  async fn transform(
    &self,
    asset: &mut MutableAsset,
    _config: &Config,
  ) -> Result<(), String> {
    asset.add_dependency(DependencyOptions {
      specifier: "./index.js".to_string(),
      specifier_type: crate::platform::public::SpecifierType::ESM,
      priority: crate::platform::public::DependencyPriority::Lazy,
      resolve_from: asset.file_path.clone(),
      imported_symbols: vec![crate::platform::public::ImportSymbolType::Namespace("".to_string())],
      bundle_behavior: crate::platform::public::BundleBehavior::Default,
    });
    return Ok(());
  }
}
