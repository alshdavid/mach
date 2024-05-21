use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

use petgraph::graph::DiGraph;

use super::AssetId;
use super::DependencyId;

pub type AssetGraphSync = Arc<RwLock<AssetGraph>>;

#[derive(Default)]
pub struct AssetGraph {
  graph: DiGraph<AssetId, DependencyId>,
}

impl AssetGraph {}
