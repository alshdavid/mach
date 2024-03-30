use crate::public::Config;
use crate::public::MutableAsset;
use crate::public::Transformer;

#[derive(Debug)]
pub struct DefaultTransformerCSS {}

impl Transformer for DefaultTransformerCSS {
  fn transform(
    &self,
    _asset: &mut MutableAsset,
    _config: &Config,
  ) -> Result<(), String> {
    return Ok(());
  }
}
