use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use swc_core::atoms::JsWord;
use swc_core::common::Globals;
use swc_core::common::Mark;
use swc_core::common::SourceMap;
use swc_core::ecma::ast::Program;
use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::codegen::Emitter;
use swc_core::ecma::transforms::base::resolver;
use swc_core::ecma::transforms::react::{self as react_transforms};
use swc_core::ecma::transforms::typescript::{self as typescript_transforms};
use swc_core::ecma::visit::FoldWith;
use swc_core::ecma::codegen::Config as SWCConfig;

use crate::default_plugins::transformers::javascript::parse_program;
use crate::default_plugins::transformers::javascript::read_imports;
use crate::public::Config;
use crate::public::DependencyOptions;

use super::collect_decls;
use super::NodeEnvReplacer;
use crate::public::{MutableAsset, Transformer};

#[derive(Debug)]
pub struct DefaultTransformerJs {}

impl Transformer for DefaultTransformerJs {
    fn transform(&self, asset: &mut MutableAsset, config: &Config) -> Result<(), String> {
      let source_map_og = Arc::new(SourceMap::default());
      let Ok(result) = parse_program(
        &asset.file_path,
        asset.get_code(),
        source_map_og.clone(),
      ) else {
        return Err(format!("SWC Parse Error"));
      };

      let program = result.program;
      let comments = result.comments;
      let source_map = source_map_og.clone();
      let file_extension = asset.file_path.extension().unwrap().to_str().unwrap().to_string();

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

      let dependencies = read_imports(&program);

      for dependency in dependencies {
        asset.add_dependency(DependencyOptions{
            specifier: dependency.specifier,
            specifier_type: dependency.specifier_type,
            priority: dependency.priority,
            resolve_from: asset.file_path.clone(),
            imported_symbols: vec![],
        });
      }

      asset.set_code(&render_program(&program, source_map_og.clone()));
      return Ok(());
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

pub fn render_program(
  module: &Program,
  cm: Arc<SourceMap>,
) -> String {
  let mut buf = vec![];
  let writer = Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None));

  let mut config = SWCConfig::default();
  config.minify = true;

  let mut emitter = Emitter {
    cfg: config,
    comments: None,
    cm: cm.clone(),
    wr: writer,
  };

  emitter.emit_program(&module).unwrap();

  return String::from_utf8(buf).unwrap();
}
