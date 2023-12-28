use std::path::PathBuf;

use swc_core::common::comments::SingleThreadedComments;
use swc_core::common::sync::Lrc;
use swc_core::common::FileName;
use swc_core::common::SourceMap;
use swc_core::ecma::ast::Module;
use swc_core::ecma::parser::lexer::Lexer;
use swc_core::ecma::parser::Parser;
use swc_core::ecma::parser::StringInput;
use swc_core::ecma::parser::Syntax;
use swc_core::ecma::parser::TsConfig;

pub fn parse(
  file_name: &PathBuf,
  code: &str,
  source_map: Lrc<SourceMap>,
) -> Result<(Module, SingleThreadedComments), String> {
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

  let module = match parser.parse_module() {
    Err(err) => return Err(format!("{:?}", err)),
    Ok(module) => module,
  };

  return Ok((module, comments));
}
