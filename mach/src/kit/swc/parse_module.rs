use std::path::PathBuf;
use std::sync::Arc;

use ad_swc_common::comments::SingleThreadedComments;
use ad_swc_common::SourceFile;
use ad_swc_common::SourceMap;
use ad_swc_ecma_ast::*;

use super::parse_program;

pub fn parse_module(
  file_name: &PathBuf,
  code: &str,
  source_map: Arc<SourceMap>,
) -> Result<ParseModuleResult, String> {
  let program = parse_program(file_name, code, source_map)?;
  match program.program {
    Program::Module(_) => println!("is module"),
    Program::Script(_) => println!("is script"),
  }
  let Program::Module(module) = program.program else {
    return Err("Incorrect type".to_string());
  };

  return Ok(ParseModuleResult {
    module,
    comments: program.comments,
    source_file: program.source_file,
  });
}

pub struct ParseModuleResult {
  pub module: Module,
  pub comments: SingleThreadedComments,
  pub source_file: Arc<SourceFile>,
}
