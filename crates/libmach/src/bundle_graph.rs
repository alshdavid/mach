use std::collections::HashMap;

use super::BundleId;
use super::DependencyId;

pub type BundleGraph = HashMap<DependencyId, BundleId>;
