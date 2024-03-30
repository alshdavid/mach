use libmach::MachConfig;
use libmach::MutableAsset;
use libmach::Transformer;

#[derive(Debug)]
pub struct DefaultTransformerNoop {}

impl Transformer for DefaultTransformerNoop {
  fn transform(
    &self,
    _asset: &mut MutableAsset,
    _config: &MachConfig,
  ) -> Result<(), String> {
    return Ok(());
  }
}
