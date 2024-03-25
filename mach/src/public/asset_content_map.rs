use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct AssetContentMap {
  pub bytes: HashMap<PathBuf, Box<Vec<u8>>>,
}

impl AssetContentMap {
  pub fn new() -> Self {
    Self::default()
  }
}
