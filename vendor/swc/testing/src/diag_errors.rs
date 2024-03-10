use std::sync::RwLock;

use swc_common::errors::Diagnostic;
use swc_common::errors::DiagnosticBuilder;
use swc_common::errors::Emitter;
use swc_common::errors::Handler;
use swc_common::errors::HandlerFlags;
use swc_common::errors::SourceMapperDyn;
use swc_common::sync::Lrc;

/// Creates a new handler for testing.
pub(crate) fn new_handler(
  _: Lrc<SourceMapperDyn>,
  treat_err_as_bug: bool,
) -> (Handler, BufferedError) {
  let e = BufferedError::default();

  let handler = Handler::with_emitter_and_flags(
    Box::new(e.clone()),
    HandlerFlags {
      treat_err_as_bug,
      can_emit_warnings: true,
      ..Default::default()
    },
  );

  (handler, e)
}

#[derive(Clone, Default)]
pub(crate) struct BufferedError(Lrc<RwLock<Vec<Diagnostic>>>);

impl Emitter for BufferedError {
  fn emit(
    &mut self,
    db: &DiagnosticBuilder,
  ) {
    self.0.write().unwrap().push((**db).clone());
  }
}

impl From<BufferedError> for Vec<Diagnostic> {
  fn from(buf: BufferedError) -> Self {
    let s = buf.0.read().unwrap();

    s.clone()
  }
}
