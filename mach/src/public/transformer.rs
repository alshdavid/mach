use std::fmt::Debug;

use crate::public::Config;

use super::MutableAsset;

pub trait Transformer: Debug + Send + Sync {
  fn transform(
    &self,
    asset: &mut MutableAsset,
    config: &Config,
  ) -> Result<(), String>;
}
