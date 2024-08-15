use crate::types::MachConfig;
use crate::types::MutableAsset;
use crate::types::Transformer;

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
