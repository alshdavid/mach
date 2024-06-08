use std::fmt::Debug;

use super::MachConfig;
use super::MutableAsset;

pub trait Transformer: Debug + Send + Sync {
  fn transform(
    &self,
    asset: &mut MutableAsset,
    config: &MachConfig,
  ) -> Result<(), String>;
}
