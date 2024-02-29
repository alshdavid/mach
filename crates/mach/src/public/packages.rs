use std::collections::HashMap;
use std::sync::Arc;

use swc_core::common::SourceMap;
use swc_core::ecma::ast::Module;

pub enum PackageType {
  JavaScript((Module, Arc<SourceMap>)),
  // CSS,
  // HTML,
  // File,
}

impl std::fmt::Debug for PackageType {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      Self::JavaScript((_, _)) => f.debug_tuple("JavaScript").finish(),
      // Self::CSS => write!(f, "CSS"),
      // Self::HTML => write!(f, "HTML"),
      // Self::File => write!(f, "File"),
    }
  }
}

pub type Packages = HashMap<String, PackageType>;
