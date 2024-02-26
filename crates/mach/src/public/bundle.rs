use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Bundle {
  pub kind: String,
  pub assets: HashSet<PathBuf>,
  pub entry_asset: PathBuf,
}

pub type Bundles = Vec<Bundle>;
