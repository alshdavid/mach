use async_trait::async_trait;

use crate::public::Config;
use crate::public::DependencyOptions;
use crate::public::MutableAsset;
use crate::public::Transformer;

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
      specifier_type: crate::public::SpecifierType::ESM,
      priority: crate::public::DependencyPriority::Lazy,
      resolve_from: asset.file_path.clone(),
      imported_symbols: vec![crate::public::ImportSymbolType::Namespace(
        "".to_string(),
      )],
      bundle_behavior: crate::public::BundleBehavior::Default,
    });
    return Ok(());
  }
}
