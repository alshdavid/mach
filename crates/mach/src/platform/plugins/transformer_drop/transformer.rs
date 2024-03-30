use libmach::MachConfig;
use libmach::MutableAsset;
use libmach::Transformer;

#[derive(Debug)]
pub struct TransformerDrop {}

impl Transformer for TransformerDrop {
  fn transform(
    &self,
    asset: &mut MutableAsset,
    _config: &MachConfig,
  ) -> Result<(), String> {
    asset.set_bytes(vec![]);
    return Ok(());
  }
}
