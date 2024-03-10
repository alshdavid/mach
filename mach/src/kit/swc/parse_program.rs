use std::path::PathBuf;
use std::sync::Arc;

use ad_swc_common::comments::SingleThreadedComments;
use ad_swc_common::FileName;
use ad_swc_common::SourceFile;
use ad_swc_common::SourceMap;
use ad_swc_ecma_ast::EsVersion;
use ad_swc_ecma_ast::Program;
use ad_swc_ecma_parser::lexer::Lexer;
use ad_swc_ecma_parser::Parser;
use ad_swc_ecma_parser::StringInput;
use ad_swc_ecma_parser::Syntax;
use ad_swc_ecma_parser::TsConfig;

pub fn parse_program(
  file_name: &PathBuf,
  code: &str,
  source_map: Arc<SourceMap>,
) -> Result<ParseProgramResult, String> {
  let source_file = source_map.new_source_file(FileName::Real(file_name.to_owned()), code.into());

  let comments = SingleThreadedComments::default();
  let syntax = {
    let mut tsconfig = TsConfig::default();
    tsconfig.tsx = true;
    Syntax::Typescript(tsconfig)
  };

  let lexer = Lexer::new(
    syntax,
    EsVersion::latest(),
    StringInput::from(&*source_file),
    Some(&comments),
  );

  let mut parser = Parser::new_from(lexer);

  let program = match parser.parse_program() {
    Err(err) => return Err(format!("{:?}", err)),
    Ok(program) => program,
  };

  return Ok(ParseProgramResult {
    program,
    comments,
    source_file,
  });
}

pub struct ParseProgramResult {
  pub program: Program,
  pub comments: SingleThreadedComments,
  pub source_file: Arc<SourceFile>,
}
