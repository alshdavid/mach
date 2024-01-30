use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

use swc_core::atoms::JsWord;
use swc_core::common::Globals;
use swc_core::common::Mark;
use swc_core::common::SourceMap;
use swc_core::ecma::ast::Program;
use swc_core::ecma::transforms::base::resolver;
use swc_core::ecma::transforms::react::{self as react_transforms};
use swc_core::ecma::transforms::typescript::{self as typescript_transforms};
use swc_core::ecma::visit::FoldWith;

use crate::default_plugins::transformers::javascript::parse_program;
use crate::default_plugins::transformers::javascript::read_imports;
use crate::public::Config;

use super::collect_decls;
use super::ImportReadResult;
use super::NodeEnvReplacer;

pub fn transformer(
  file_path: &PathBuf,
  contents: &str,
  source_map: Arc<SourceMap>,
  config: &Config,
) -> Result<(Program, Vec<ImportReadResult>), String> {
  let Ok(result) = parse_program(file_path, contents, source_map.clone()) else {
    return Err(format!("SWC Parse Error"));
  };

  let program = result.program;
  let comments = result.comments;
  let source_map = source_map.clone();
  let file_extension = file_path.extension().unwrap().to_str().unwrap().to_string();
  let dependencies = read_imports(&program);

  let program = swc_core::common::GLOBALS.set(&Globals::new(), move || {
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
    }

    if file_extension == "ts" {
      program = program.fold_with(&mut typescript_transforms::strip(top_level_mark));
    }

    return program;
  });

  return Ok((program, dependencies));
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
