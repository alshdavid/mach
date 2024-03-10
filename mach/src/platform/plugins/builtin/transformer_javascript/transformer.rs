use std::sync::Arc;

use ad_swc_common::Globals;
use ad_swc_common::Mark;
use ad_swc_common::SourceMap;
use ad_swc_ecma_transforms_base::resolver;
use ad_swc_ecma_transforms_react::{self as react_transforms};
use ad_swc_ecma_transforms_typescript::{self as typescript_transforms};
use ad_swc_ecma_visit::FoldWith;
use async_trait::async_trait;

use crate::kit::swc::parse_program;
use crate::kit::swc::render_program;
use crate::public::Config;
use crate::public::DependencyOptions;
use crate::public::MutableAsset;
use crate::public::Transformer;

// use super::collect_decls;
use super::read_imports_exports;
use super::NodeEnvReplacer;

#[derive(Debug)]
pub struct DefaultTransformerJavaScript {}

#[async_trait]
impl Transformer for DefaultTransformerJavaScript {
  async fn transform(
    &self,
    asset: &mut MutableAsset,
    config: &Config,
  ) -> Result<(), String> {
    return ad_swc_common::GLOBALS.set(&Globals::new(), move || {
      let source_map_og = Arc::new(SourceMap::default());
      let code = asset.get_code();

      let Ok(result) = parse_program(&asset.file_path, &code, source_map_og.clone()) else {
        return Err(format!("SWC Parse Error"));
      };

      let program = result.program;

      let comments = result.comments;
      let source_map = source_map_og.clone();
      let file_extension = asset
        .file_path
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

      let top_level_mark = Mark::fresh(Mark::root());
      let unresolved_mark = Mark::fresh(Mark::root());

      let mut program = program.fold_with(&mut resolver(unresolved_mark, top_level_mark, false));

      // let decls = collect_decls(&program);

      program = program.fold_with(&mut NodeEnvReplacer { env: &config.env });

      if file_extension == "jsx" {
        program = program.fold_with(&mut react_transforms::react(
          source_map.clone(),
          Some(&comments),
          react_transforms::Options::default(),
          top_level_mark,
          unresolved_mark,
        ));
        *asset.kind = "js".to_string();
      }

      if file_extension == "tsx" {
        program = program.fold_with(&mut typescript_transforms::tsx(
          source_map.clone(),
          Default::default(),
          typescript_transforms::TsxConfig::default(),
          Some(&comments),
          top_level_mark,
        ));

        program = program.fold_with(&mut typescript_transforms::strip(top_level_mark));

        *asset.kind = "js".to_string();
      }

      if file_extension == "ts" {
        program = program.fold_with(&mut typescript_transforms::strip(top_level_mark));
        *asset.kind = "js".to_string();
      }

      let (dependencies, _) = read_imports_exports(&program, &asset.file_path);

      for dependency in dependencies {
        asset.add_dependency(DependencyOptions {
          specifier: dependency.specifier,
          specifier_type: dependency.specifier_type,
          priority: dependency.priority,
          resolve_from: asset.file_path.clone(),
          imported_symbols: dependency.imported_symbols,
          bundle_behavior: crate::public::BundleBehavior::Default,
        });
      }

      asset.set_code(&render_program(&program, source_map_og.clone()));

      return Ok(());
    });
  }
}
