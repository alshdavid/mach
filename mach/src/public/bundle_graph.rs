use std::collections::HashMap;

use super::{BundleId, DependencyId};

pub type BundleGraph = HashMap<DependencyId, BundleId>;
