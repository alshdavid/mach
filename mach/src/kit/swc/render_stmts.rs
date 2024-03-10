use std::sync::Arc;

use ad_swc_common::SourceMap;
use ad_swc_common::Span;
use ad_swc_ecma_ast::*;
use ad_swc_ecma_codegen::text_writer::JsWriter;
use ad_swc_ecma_codegen::Config as SWCConfig;
use ad_swc_ecma_codegen::Emitter;

pub fn render_stmts(
  stmts: &Vec<Stmt>,
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

  let script = Script {
    span: Span::default(),
    body: stmts.clone(),
    shebang: None,
  };

  emitter.emit_script(&script).unwrap();

  return String::from_utf8(buf).unwrap();
}
