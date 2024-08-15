use std::collections::HashMap;

use super::Dependency;
use super::DependencyId;

pub type DependencyMap = HashMap<DependencyId, Dependency>;
