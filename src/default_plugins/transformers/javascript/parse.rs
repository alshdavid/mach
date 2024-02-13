use std::path::PathBuf;
use std::sync::Arc;

use swc_core::common::comments::SingleThreadedComments;
use swc_core::common::FileName;
use swc_core::common::SourceFile;
use swc_core::common::SourceMap;
use swc_core::ecma::ast::Program;
use swc_core::ecma::parser::lexer::Lexer;
use swc_core::ecma::parser::Parser;
use swc_core::ecma::parser::StringInput;
use swc_core::ecma::parser::Syntax;
use swc_core::ecma::parser::TsConfig;

// pub fn parse_module(
//   file_name: &PathBuf,
//   code: &str,
//   source_map: Lrc<SourceMap>,
// ) -> Result<(Module, SingleThreadedComments), String> {
//   let source_file = source_map.new_source_file(FileName::Real(file_name.to_owned()), code.into());

//   let comments = SingleThreadedComments::default();
//   let syntax = {
//     let mut tsconfig = TsConfig::default();
//     tsconfig.tsx = true;
//     Syntax::Typescript(tsconfig)
//   };

//   let lexer = Lexer::new(
//     syntax,
//     swc_core::ecma::ast::EsVersion::latest(),
//     StringInput::from(&*source_file),
//     Some(&comments),
//   );

//   let mut parser = Parser::new_from(lexer);

//   let module = match parser.parse_module() {
//     Err(err) => return Err(format!("{:?}", err)),
//     Ok(module) => module,
//   };

//   return Ok((module, comments));
// }

pub fn parse_program(
  file_name: &PathBuf,
  code: &str,
  source_map: Arc<SourceMap>,
) -> Result<ParseResult, String> {
  let source_file = source_map.new_source_file(FileName::Real(file_name.to_owned()), code.into());

  let comments = SingleThreadedComments::default();
  let syntax = {
    let mut tsconfig = TsConfig::default();
    tsconfig.tsx = true;
    Syntax::Typescript(tsconfig)
  };

  let lexer = Lexer::new(
    syntax,
    swc_core::ecma::ast::EsVersion::latest(),
    StringInput::from(&*source_file),
    Some(&comments),
  );

  let mut parser = Parser::new_from(lexer);

  let program = match parser.parse_program() {
    Err(err) => return Err(format!("{:?}", err)),
    Ok(module) => module,
  };

  return Ok(ParseResult {
    program,
    comments,
    source_file,
  });
}

pub struct ParseResult {
  pub program: Program,
  pub comments: SingleThreadedComments,
  pub source_file: Arc<SourceFile>,
}
