//! Usage: cargo run --example ts_to_js path/to/ts
//!
//! This program will emit output to stdout.

use std::env;
use std::path::Path;

use ad_swc_ecma_transforms_typescript as swc_ecma_transforms_typescript;
use swc_common::comments::SingleThreadedComments;
use swc_common::errors::ColorConfig;
use swc_common::errors::Handler;
use swc_common::sync::Lrc;
use swc_common::Globals;
use swc_common::Mark;
use swc_common::SourceMap;
use swc_common::GLOBALS;
use swc_common::{self};
use swc_ecma_codegen::text_writer::JsWriter;
use swc_ecma_codegen::Emitter;
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::Parser;
use swc_ecma_parser::StringInput;
use swc_ecma_parser::Syntax;
use swc_ecma_parser::TsConfig;
use swc_ecma_transforms_base::fixer::fixer;
use swc_ecma_transforms_base::hygiene::hygiene;
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_typescript::strip;
use swc_ecma_visit::FoldWith;

fn main() {
  let cm: Lrc<SourceMap> = Default::default();
  let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

  // Real usage
  // let fm = cm
  //     .load_file(Path::new("test.js"))
  //     .expect("failed to load test.js");

  let input = env::args()
    .nth(1)
    .expect("please provide the path of input typescript file");

  let fm = cm
    .load_file(Path::new(&input))
    .expect("failed to load input typescript file");

  let comments = SingleThreadedComments::default();

  let lexer = Lexer::new(
    Syntax::Typescript(TsConfig {
      tsx: input.ends_with(".tsx"),
      ..Default::default()
    }),
    Default::default(),
    StringInput::from(&*fm),
    Some(&comments),
  );

  let mut parser = Parser::new_from(lexer);

  for e in parser.take_errors() {
    e.into_diagnostic(&handler).emit();
  }

  let module = parser
    .parse_module()
    .map_err(|e| e.into_diagnostic(&handler).emit())
    .expect("failed to parse module.");

  let globals = Globals::default();
  GLOBALS.set(&globals, || {
    let unresolved_mark = Mark::new();
    let top_level_mark = Mark::new();

    // Optionally transforms decorators here before the resolver pass
    // as it might produce runtime declarations.

    // Conduct identifier scope analysis
    let module = module.fold_with(&mut resolver(unresolved_mark, top_level_mark, true));

    // Remove typescript types
    let module = module.fold_with(&mut strip(top_level_mark));

    // Fix up any identifiers with the same name, but different contexts
    let module = module.fold_with(&mut hygiene());

    // Ensure that we have enough parenthesis.
    let module = module.fold_with(&mut fixer(Some(&comments)));

    let mut buf = vec![];
    {
      let mut emitter = Emitter {
        cfg: swc_ecma_codegen::Config::default(),
        cm: cm.clone(),
        comments: Some(&comments),
        wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
      };

      emitter.emit_module(&module).unwrap();
    }

    println!("{}", String::from_utf8(buf).expect("non-utf8?"));
  })
}
