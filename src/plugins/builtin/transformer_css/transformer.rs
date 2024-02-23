use async_trait::async_trait;

use crate::public::Config;
use crate::public::MutableAsset;
use crate::public::Transformer;

#[derive(Debug)]
pub struct DefaultTransformerCSS {}

#[async_trait]
impl Transformer for DefaultTransformerCSS {
  async fn transform(
    &self,
    asset: &mut MutableAsset,
    config: &Config,
  ) -> Result<(), String> {
    return Ok(());
  }
}
