use swc_core::common::sync::Lrc;
use swc_core::common::Globals;
use swc_core::common::Mark;
use swc_core::common::SourceMap;
use swc_core::ecma::ast::*;
use swc_core::ecma::transforms::typescript::{self as typescript_transforms};
use swc_core::ecma::visit::FoldWith;

pub fn transform_ts(_source_map: Lrc<SourceMap>, module: Module) -> Module {
  return swc_core::common::GLOBALS.set(&Globals::new(), move || {
    let top_level_mark = Mark::fresh(Mark::root());

    let program = Program::Module(module).fold_with(&mut typescript_transforms::strip(top_level_mark));

    let Program::Module(m) = program else { panic!("")};
    return m;
  });
}
