use swc_core::common::comments::SingleThreadedComments;
use swc_core::common::sync::Lrc;
use swc_core::common::Globals;
use swc_core::common::Mark;
use swc_core::common::SourceMap;
use swc_core::ecma::ast::*;
use swc_core::ecma::transforms::react::{self as react_transforms};
use swc_core::ecma::visit::FoldWith;

pub fn transform_jsx(source_map: Lrc<SourceMap>, mut module: Module) -> Module {
  return swc_core::common::GLOBALS.set(&Globals::new(), move || {
    let top_level_mark = Mark::fresh(Mark::root());
    let unresolved_mark = Mark::fresh(Mark::root());
    let comments: Option<SingleThreadedComments> = None;

    module = module.fold_with(&mut react_transforms::react(
      source_map.clone(),
      comments,
      react_transforms::Options::default(),
      top_level_mark,
      unresolved_mark,
    ));

    return module;
  });
}
