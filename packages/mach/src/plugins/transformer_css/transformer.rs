use crate::public::MachConfig;
use crate::public::MutableAsset;
use crate::public::Transformer;

#[derive(Debug)]
pub struct TransformerCSS {}

impl Transformer for TransformerCSS {
  fn transform(
    &self,
    _asset: &mut MutableAsset,
    _config: &MachConfig,
  ) -> anyhow::Result<()> {
    return Ok(());
  }
}
