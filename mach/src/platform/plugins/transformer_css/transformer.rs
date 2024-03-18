use async_trait::async_trait;

use libmach::Config;
use libmach::MutableAsset;
use libmach::Transformer;

#[derive(Debug)]
pub struct TransformerCSS {}

#[async_trait]
impl Transformer for TransformerCSS {
  async fn transform(
    &self,
    _asset: &mut MutableAsset,
    _config: &Config,
  ) -> Result<(), String> {
    return Ok(());
  }
}
