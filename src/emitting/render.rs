use swc_core::common::sync::Lrc;
use swc_core::common::SourceMap;
use swc_core::ecma::ast::Script;
use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::codegen::Config;
use swc_core::ecma::codegen::Emitter;

pub fn render(module: &Script, cm: Lrc<SourceMap>) -> String {
  let mut buf = vec![];
  let writer = Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None));

  let mut config = Config::default();
  config.minify = false;

  let mut emitter = Emitter {
    cfg: config,
    comments: None,
    cm: cm.clone(),
    wr: writer,
  };

  emitter.emit_script(&module).unwrap();

  return String::from_utf8(buf).unwrap();
}
