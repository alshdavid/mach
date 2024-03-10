use std::path::PathBuf;
use std::sync::Arc;

use ad_swc_common::comments::SingleThreadedComments;
use ad_swc_common::SourceFile;
use ad_swc_common::SourceMap;
use ad_swc_ecma_ast::*;

use super::parse_program;

pub fn parse_script(
  file_name: &PathBuf,
  code: &str,
  source_map: Arc<SourceMap>,
) -> Result<ParseScriptResult, String> {
  let program = parse_program(file_name, code, source_map)?;
  let Program::Script(script) = program.program else {
    return Err("Incorrect type".to_string());
  };

  return Ok(ParseScriptResult {
    script,
    comments: program.comments,
    source_file: program.source_file,
  });
}

pub struct ParseScriptResult {
  pub script: Script,
  pub comments: SingleThreadedComments,
  pub source_file: Arc<SourceFile>,
}
