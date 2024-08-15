use crate::public::MachConfig;
use crate::public::MutableAsset;
use crate::public::Transformer;

#[derive(Debug)]
pub struct TransformerDrop {}

impl Transformer for TransformerDrop {
  fn transform(
    &self,
    asset: &mut MutableAsset,
    _config: &MachConfig,
  ) -> anyhow::Result<()> {
    asset.set_bytes(vec![]);
    return Ok(());
  }
}
