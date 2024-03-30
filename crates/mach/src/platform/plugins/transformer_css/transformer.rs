use libmach::MachConfig;
use libmach::MutableAsset;
use libmach::Transformer;

#[derive(Debug)]
pub struct TransformerCSS {}

impl Transformer for TransformerCSS {
  fn transform(
    &self,
    _asset: &mut MutableAsset,
    _config: &MachConfig,
  ) -> Result<(), String> {
    return Ok(());
  }
}
