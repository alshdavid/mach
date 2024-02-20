use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Bundle {
  pub assets: HashSet<PathBuf>,
  pub entry_asset: PathBuf,
}

pub type Bundles = Vec<Bundle>;
