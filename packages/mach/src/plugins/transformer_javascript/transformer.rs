use std::sync::Arc;

use swc_core::common::Globals;
use swc_core::common::Mark;
use swc_core::common::SourceMap;
use swc_core::ecma::transforms::base::resolver;
use swc_core::ecma::transforms::react::{self as react_transforms};
use swc_core::ecma::transforms::typescript::{self as typescript_transforms};
use swc_core::ecma::visit::FoldWith;

use super::create_dependencies::create_dependencies;
use super::extract_imports_exports::extract_imports_exports;
use super::replace_node_env::replace_node_env;
use crate::kit::swc::parse_program;
use crate::kit::swc::render_program;
use crate::public::BundleBehavior;
use crate::public::DependencyOptions;
use crate::public::MachConfig;
use crate::public::MutableAsset;
use crate::public::Transformer;

#[derive(Debug)]
pub struct TransformerJavaScript {}

impl Transformer for TransformerJavaScript {
  fn transform(
    &self,
    asset: &mut MutableAsset,
    config: &MachConfig,
  ) -> Result<(), String> {
    return swc_core::common::GLOBALS.set(&Globals::new(), move || {
      let source_map_og = Arc::new(SourceMap::default());
      let code = asset.get_code();

      let Ok(result) = parse_program(asset.file_path, &code, source_map_og.clone()) else {
        return Err(format!("SWC Parse Error"));
      };

      let mut program = result.program;

      let comments = result.comments;
      let source_map = source_map_og.clone();

      let top_level_mark = Mark::fresh(Mark::root());
      let unresolved_mark = Mark::fresh(Mark::root());

      program = program.fold_with(&mut resolver(unresolved_mark, top_level_mark, false));
      program = replace_node_env(program, &config.env);

      // Strip Types
      if *asset.kind == "tsx" {
        program = program.fold_with(&mut typescript_transforms::tsx(
          source_map.clone(),
          Default::default(),
          typescript_transforms::TsxConfig::default(),
          Some(&comments),
          top_level_mark,
        ));

        *asset.kind = "jsx".to_string();
      }

      // Strip Types
      if *asset.kind == "ts" {
        program = program.fold_with(&mut typescript_transforms::typescript(
          Default::default(),
          top_level_mark,
        ));

        *asset.kind = "js".to_string();
      }

      // Convert JSX
      if *asset.kind == "jsx" {
        program = program.fold_with(&mut react_transforms::react(
          source_map.clone(),
          Some(&comments),
          react_transforms::Options::default(),
          top_level_mark,
          unresolved_mark,
        ));
      }

      // Dead code elimination
      // Turned off for now because React doesn't like it

      // program = program.fold_with(&mut ecma_simplify::expr_simplifier(unresolved_mark, Default::default()));
      // program = program.fold_with(&mut ecma_simplify::dead_branch_remover(unresolved_mark));

      *asset.kind = "js".to_string();

      let linking_symbols = extract_imports_exports(&program);
      let dependencies = create_dependencies(&linking_symbols);

      for dependency in dependencies {
        asset.add_dependency(DependencyOptions {
          resolve_from: asset.file_path.to_path_buf(),
          ..dependency
        });
      }

      *asset.linking_symbols = linking_symbols;

      asset.set_code(&render_program(&program, source_map_og.clone()));

      return Ok(());
    });
  }
}
