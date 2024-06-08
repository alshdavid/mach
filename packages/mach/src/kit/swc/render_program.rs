use std::sync::Arc;

use swc_core::common::SourceMap;
use swc_core::ecma::ast::*;
use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::codegen::Config as SWCConfig;
use swc_core::ecma::codegen::Emitter;

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
