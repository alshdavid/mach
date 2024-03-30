use libmach::MachConfig;
use libmach::MutableAsset;
use libmach::Transformer;

#[derive(Debug)]
pub struct DefaultTransformerCSS {}

impl Transformer for DefaultTransformerCSS {
  fn transform(
    &self,
    _asset: &mut MutableAsset,
    _config: &MachConfig,
  ) -> Result<(), String> {
    return Ok(());
  }
}
