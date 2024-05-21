use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

use super::Dependency;
use super::DependencyId;

pub type DependencyMap = HashMap<DependencyId, Dependency>;
pub type DependencyMapSync = Arc<RwLock<DependencyMap>>;
