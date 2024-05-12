use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use super::BundleId;
use super::DependencyId;

pub type BundleGraphSync = Arc<RwLock<HashMap<DependencyId, BundleId>>>;

pub type BundleGraph = HashMap<DependencyId, BundleId>;
