use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use async_trait::async_trait;
use swc_core::atoms::JsWord;
use swc_core::common::Globals;
use swc_core::common::Mark;
use swc_core::common::SourceMap;
use swc_core::ecma::transforms::base::resolver;
use swc_core::ecma::transforms::react::{self as react_transforms};
use swc_core::ecma::transforms::typescript::{self as typescript_transforms};
use swc_core::ecma::visit::FoldWith;

use crate::kit::swc::parse_program;
use crate::kit::swc::render_program;
use crate::public::Config;
use crate::public::DependencyOptions;
use crate::public::MutableAsset;
use crate::public::Transformer;

use super::collect_decls;
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
    return swc_core::common::GLOBALS.set(&Globals::new(), move || {
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

      let decls = collect_decls(&program);

      program = program.fold_with(&mut NodeEnvReplacer {
        replace_env: true,
        env: &get_env(&config.env),
        is_browser: true,
        decls: &decls,
        used_env: &mut HashSet::new(),
        source_map: &source_map.clone(),
        unresolved_mark,
      });

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
        program = program.fold_with(&mut typescript_transforms::strip(top_level_mark));

        program = program.fold_with(&mut typescript_transforms::tsx(
          source_map.clone(),
          Default::default(),
          typescript_transforms::TsxConfig::default(),
          Some(&comments),
          top_level_mark,
        ));

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

fn get_env(config_env: &HashMap<String, String>) -> HashMap<JsWord, JsWord> {
  let mut env = HashMap::<JsWord, JsWord>::new();
  for (key, value) in config_env.iter() {
    env.insert(
      JsWord::from(key.to_string()),
      JsWord::from(value.to_string()),
    );
  }
  return env;
}
