use libmach::MachConfig;
use libmach::MutableAsset;
use libmach::Transformer;

#[derive(Debug)]
pub struct NoopTransformer {}

impl Transformer for NoopTransformer {
  fn transform(
    &self,
    _: &mut MutableAsset,
    _: &MachConfig,
  ) -> Result<(), String> {
    return Ok(());
  }
}
