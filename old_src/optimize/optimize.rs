use swc_core::common::comments::Comments;
use swc_core::common::sync::Lrc;
use swc_core::common::Globals;
use swc_core::common::Mark;
use swc_core::common::SourceMap;
use swc_core::ecma::ast::EsVersion;
use swc_core::ecma::ast::Program;
use swc_core::ecma::minifier::{self};
use swc_core::ecma::transforms::base::resolver;
use swc_core::ecma::visit::FoldWith;

use crate::package::PackagedBundles;

pub fn optimize(
  packaged_bundles: PackagedBundles,
  source_map: Lrc<SourceMap>,
) -> Result<(Lrc<SourceMap>, PackagedBundles), String> {
  let mut optimized_packages = PackagedBundles::new();

  for (out_file, mut module) in packaged_bundles {
    let source_map = source_map.clone();
    let module = swc_core::common::GLOBALS.set(&Globals::new(), move || {
      // return swc_core::common::errors::HANDLER.set(&handler, || {
      let comments: Option<&dyn Comments> = None;
      let top_level_mark = Mark::fresh(Mark::root());
      let unresolved_mark = Mark::fresh(Mark::root());

      module = module.fold_with(&mut resolver(unresolved_mark, top_level_mark, false));

      // module = module.fold_with(&mut simplifier(unresolved_mark, Config{
      // 	dce: Default::default(),
      // 	inlining: Default::default(),
      // 	expr: Default::default()
      // }));

      let compress_options_default = minifier::option::CompressOptions::default();

      let compress_options = minifier::option::CompressOptions {
        arguments: false,
        arrows: true,
        bools: true,
        bools_as_ints: false,
        collapse_vars: true,
        comparisons: true,
        computed_props: true,
        conditionals: true,
        dead_code: false,
        directives: true,
        drop_console: false,
        drop_debugger: true,
        ecma: EsVersion::Es2022,
        evaluate: true,
        expr: false,
        global_defs: compress_options_default.global_defs,
        hoist_fns: false,
        hoist_props: true,
        hoist_vars: false,
        ie8: false,
        if_return: true,
        inline: 0, //compress_options_default.inline,
        join_vars: false,
        keep_classnames: false,
        keep_fargs: true,
        keep_fnames: false,
        keep_infinity: false,
        loops: true,
        module: false, //compress_options_default.module,
        negate_iife: true,
        passes: 3,    //compress_options_default.passes,
        props: false, ////compress_options_default.props,
        pure_getters: compress_options_default.pure_getters,
        pure_funcs: compress_options_default.pure_funcs,
        reduce_fns: false,
        reduce_vars: false,
        sequences: 0, //compress_options_default.sequences,
        side_effects: true,
        switches: true,
        top_retain: vec![], //compress_options_default.top_retain,
        top_level: None,    //compress_options_default.top_level,
        typeofs: true,
        unsafe_passes: false, //compress_options_default.unsafe_passes,
        unsafe_arrows: false,
        unsafe_comps: false,
        unsafe_function: false,
        unsafe_math: false,
        unsafe_symbols: false,
        unsafe_methods: false,
        unsafe_proto: false,
        unsafe_regexp: false,
        unsafe_undefined: false,
        unused: true,
        const_to_let: true,
        pristine_globals: true,
      };

      // dbg!(&compress_options);

      let _mangle_options = minifier::option::MangleOptions {
        props: None,
        top_level: Some(false),
        keep_class_names: false,
        keep_fn_names: false,
        keep_private_props: false,
        ie8: false,
        safari10: false,
        reserved: vec!["React".into()],
        eval: false,
      };

      let Program::Module(module) = minifier::optimize(
        Program::Module(module),
        source_map.clone(),
        comments,
        None,
        &minifier::option::MinifyOptions {
          rename: true,
          compress: Some(compress_options),
          mangle: None, //Some(mangle_options),
          wrap: false,
          enclose: false,
        },
        &minifier::option::ExtraOptions {
          unresolved_mark,
          top_level_mark,
        },
      ) else {
        println!("Hi");
        panic!("Not implemented");
      };

      return module;
      // });
    });

    optimized_packages.insert(out_file, module);
  }

  return Ok((Lrc::new(SourceMap::default()), optimized_packages));
}
