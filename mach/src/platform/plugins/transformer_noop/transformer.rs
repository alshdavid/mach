use async_trait::async_trait;

use libmach::Config;
use libmach::MutableAsset;
use libmach::Transformer;

#[derive(Debug)]
pub struct DefaultTransformerNoop {}

#[async_trait]
impl Transformer for DefaultTransformerNoop {
  async fn transform(
    &self,
    _asset: &mut MutableAsset,
    _config: &Config,
  ) -> Result<(), String> {
    return Ok(());
  }
}
