use libmach::MachConfig;
use libmach::MutableAsset;
use libmach::Transformer;

#[derive(Debug)]
pub struct DenoTransformer {}

impl Transformer for DenoTransformer {
  fn transform(
    &self,
    _: &mut MutableAsset,
    _: &MachConfig,
  ) -> Result<(), String> {
    return Ok(());
  }
}
