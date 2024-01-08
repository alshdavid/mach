use swc_core::common::SourceMap;

use crate::linking::DependencyIndex;

use super::DependencyMap;
use super::AssetMap;

pub type AssetMapRef = Option<AssetMap>;
pub type DependencyMapRef = Option<DependencyMap>;
pub type DependencyIndexRef = Option<DependencyIndex>;
pub type SourceMapRef = Option<SourceMap>;