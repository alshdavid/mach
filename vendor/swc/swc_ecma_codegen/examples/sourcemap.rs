/// Use memory allocator
extern crate swc_malloc;

use std::env;
use std::fs::File;
use std::fs::{self};
use std::io::BufWriter;
use std::path::Path;
use std::time::Instant;

use ad_swc_ecma_codegen::text_writer::JsWriter;
use ad_swc_ecma_codegen::Emitter;
use swc_common::input::SourceFileInput;
use swc_ecma_ast::*;
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::Parser;
use swc_ecma_parser::Syntax;

fn parse_and_gen(entry: &Path) {
  testing::run_test2(false, |cm, _| {
    let fm = cm.load_file(entry).unwrap();

    let lexer = Lexer::new(
      Syntax::Typescript(Default::default()),
      EsVersion::latest(),
      SourceFileInput::from(&*fm),
      None,
    );
    let mut parser = Parser::new_from(lexer);
    let m = parser
      .parse_module()
      .expect("failed to parse input as a module");

    let mut code = vec![];
    let mut srcmap = vec![];

    {
      let mut emitter = Emitter {
        cfg: Default::default(),
        cm: cm.clone(),
        comments: None,
        wr: JsWriter::new(cm.clone(), "\n", &mut code, Some(&mut srcmap)),
      };

      emitter.emit_module(&m).unwrap();
    }

    let srcmap = cm.build_source_map(&srcmap);

    fs::write("output.js", &code).unwrap();

    let srcmap_file = File::create("output.js.map").unwrap();
    let srcmap_wr = BufWriter::new(srcmap_file);
    srcmap.to_writer(srcmap_wr).unwrap();

    Ok(())
  })
  .expect("failed to process a module");
}

/// Usage: ./scripts/instruements path/to/input/file
fn main() {
  let main_file = env::args().nth(1).unwrap();

  let start = Instant::now();
  parse_and_gen(Path::new(&main_file));
  let dur = start.elapsed();
  println!("Took {:?}", dur);
}
