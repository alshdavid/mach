use libmach::Config;
use libmach::MutableAsset;
use libmach::Transformer;

#[derive(Debug)]
pub struct TransformerCSS {}

impl Transformer for TransformerCSS {
  fn transform(
    &self,
    _asset: &mut MutableAsset,
    _config: &Config,
  ) -> Result<(), String> {
    return Ok(());
  }
}
