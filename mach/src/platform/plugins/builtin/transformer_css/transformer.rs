use async_trait::async_trait;

use crate::platform::public::Config;
use crate::platform::public::MutableAsset;
use crate::platform::public::Transformer;

#[derive(Debug)]
pub struct DefaultTransformerCSS {}

#[async_trait]
impl Transformer for DefaultTransformerCSS {
  async fn transform(
    &self,
    _asset: &mut MutableAsset,
    _config: &Config,
  ) -> Result<(), String> {
    return Ok(());
  }
}
