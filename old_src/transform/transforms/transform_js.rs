use swc_core::common::sync::Lrc;
use swc_core::common::SourceMap;
use swc_core::ecma::ast::*;

pub fn transform_js(_source_map: Lrc<SourceMap>, module: Module) -> Module {
  return module;
}
