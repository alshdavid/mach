use crate::public::Config;
use crate::public::MutableAsset;
use crate::public::Transformer;

#[derive(Debug)]
pub struct DefaultTransformerDrop {}

impl Transformer for DefaultTransformerDrop {
  fn transform(
    &self,
    asset: &mut MutableAsset,
    _config: &Config,
  ) -> Result<(), String> {
    asset.set_bytes(vec![]);
    return Ok(());
  }
}
