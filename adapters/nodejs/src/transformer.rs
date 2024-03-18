use async_trait::async_trait;
use libmach::Config;
use libmach::MutableAsset;
use libmach::Transformer;

#[derive(Debug)]
pub struct NodejsTransformer {}

#[async_trait]
impl Transformer for NodejsTransformer {
  async fn transform(
    &self,
    _: &mut MutableAsset,
    _: &Config,
  ) -> Result<(), String> {
    return Ok(());
  }
}