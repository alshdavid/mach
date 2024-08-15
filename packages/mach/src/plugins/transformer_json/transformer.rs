use crate::public::MachConfig;
use crate::public::MutableAsset;
use crate::public::Transformer;

#[derive(Debug)]
pub struct TransformerJson {}

impl Transformer for TransformerJson {
  fn transform(
    &self,
    asset: &mut MutableAsset,
    _config: &MachConfig,
  ) -> anyhow::Result<()> {
    let code = asset.get_code();

    asset.set_code(&format!("export default ({})", code));
    *asset.kind = "js".to_string();

    return Ok(());
  }
}
