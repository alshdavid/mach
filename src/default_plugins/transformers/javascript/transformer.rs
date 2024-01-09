use std::collections::HashMap;
use std::collections::HashSet;

use swc_core::atoms::JsWord;
use swc_core::common::Globals;
use swc_core::common::Mark;
use swc_core::ecma::transforms::base::resolver;
use swc_core::ecma::transforms::react::{self as react_transforms};
use swc_core::ecma::transforms::typescript::{self as typescript_transforms};
use swc_core::ecma::visit::FoldWith;

use crate::default_plugins::transformers::javascript::parse_program;
use crate::default_plugins::transformers::javascript::read_imports;
use crate::public::Asset;
use crate::public::TransformResult;
use crate::public::Transformer;
use crate::public::TransformerContext;

use super::collect_decls;
use super::NodeEnvReplacer;

pub struct JavaScriptTransformer;

impl Transformer for JavaScriptTransformer {
  fn transform(&self, ctx: &TransformerContext, asset: &mut Asset) -> TransformResult {
    let Asset::Unknown(asset) = asset else {
      return TransformResult::Next;
    };

    let Ok(mut result) = parse_program(&asset.file_path, &asset.contents, ctx.source_map.clone()) else {
      return TransformResult::Err(format!("SWC Parse Error"));
    };

    let program = result.program;
    let comments = result.comments;
    let source_map = ctx.source_map.clone();
    let file_extension = asset.file_path.extension().unwrap().to_str().unwrap().to_string();

    result.program = swc_core::common::GLOBALS.set(&Globals::new(), move || {
      let top_level_mark = Mark::fresh(Mark::root());
      let unresolved_mark = Mark::fresh(Mark::root());

      let mut program = program.fold_with(&mut resolver(unresolved_mark, top_level_mark, false));

      let decls = collect_decls(&program);
      
      program = program.fold_with(&mut NodeEnvReplacer {
        replace_env: true,
        env: &get_env(&ctx.config.env),
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
      }

      if file_extension == "tsx" {
        program = program
          .fold_with(&mut typescript_transforms::strip(top_level_mark));

        program = program.fold_with(&mut typescript_transforms::tsx(
          source_map.clone(),
          Default::default(),
          typescript_transforms::TsxConfig::default(),
          Some(&comments),
          top_level_mark,
        ));
      }

      if file_extension == "ts" {
        program = program.fold_with(&mut typescript_transforms::strip(top_level_mark));
      }

      return program;
    });

    for dependency in read_imports(&result.program) {
      ctx.add_dependency(&dependency.specifier, dependency.kind);
    }

    return TransformResult::Convert(Asset::JavaScript(asset.to_javascript(result.program)));
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
