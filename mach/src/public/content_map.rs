#![allow(dead_code)]
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use tokio::sync::RwLock;

use super::AssetId;

pub type ContentMapSync = Arc<RwLock<ContentMap>>;

#[derive(Default)]
pub struct ContentMap {
  contents: HashMap<AssetId, Vec<u8>>,
}

impl ContentMap {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Debug for ContentMap {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let mut map = BTreeMap::<String, usize>::new();
    for (k, s) in &self.contents {
      map.insert(format!("AssetId({})", k.0.to_string()), s.len());
    }
    f.debug_map().entries(&map).finish()
  }
}
