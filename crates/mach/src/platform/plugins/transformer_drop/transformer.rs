use libmach::MachConfig;
use libmach::MutableAsset;
use libmach::Transformer;

#[derive(Debug)]
pub struct DefaultTransformerDrop {}

impl Transformer for DefaultTransformerDrop {
  fn transform(
    &self,
    asset: &mut MutableAsset,
    _config: &MachConfig,
  ) -> Result<(), String> {
    asset.set_bytes(vec![]);
    return Ok(());
  }
}
