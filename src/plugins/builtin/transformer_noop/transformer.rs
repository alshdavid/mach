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
    config: &Config,
  ) -> Result<(), String> {
    return Ok(());
  }
}
