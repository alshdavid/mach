use std::sync::Arc;

use ad_swc_common::SourceMap;
use ad_swc_ecma_ast::*;
use ad_swc_ecma_codegen::text_writer::JsWriter;
use ad_swc_ecma_codegen::Config as SWCConfig;
use ad_swc_ecma_codegen::Emitter;

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
