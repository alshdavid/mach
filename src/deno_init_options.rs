use std::path::PathBuf;
use std::thread;

use deno_ast::ModuleSpecifier;
use deno_core::v8;
use deno_core::Extension;
use deno_core::SharedArrayBufferStore;
use deno_io::Stdio;
use deno_terminal::colors;

pub struct DenoInitOptions {
  pub cwd: PathBuf,
  pub main_module: Option<String>,
  /// Sets `Deno.args` in JS runtime.
  pub args: Vec<String>,
  pub argv0: Option<String>,
  pub cpu_count: usize,
  pub enable_op_summary_metrics: bool,
  pub locale: String,
  pub location: Option<ModuleSpecifier>,
  pub no_color: bool,
  pub user_agent: String,
  pub inspect: bool,
  /// JsRuntime extensions, not to be confused with ES modules.
  pub extensions: Vec<Extension>,
  /// Should op registration be skipped?
  pub skip_op_registration: bool,
  /// Optional isolate creation parameters, such as heap limits.
  pub create_params: Option<v8::CreateParams>,
  pub unsafely_ignore_certificate_errors: Option<Vec<String>>,
  // If true, the worker will wait for inspector session and break on first
  // statement of user code. Takes higher precedence than
  // `should_wait_for_inspector_session`.
  pub should_break_on_first_statement: bool,
  // If true, the worker will wait for inspector session before executing
  // user code.
  pub should_wait_for_inspector_session: bool,
  /// The store to use for transferring SharedArrayBuffers between isolates.
  /// If multiple isolates should have the possibility of sharing
  /// SharedArrayBuffers, they should use the same [SharedArrayBufferStore]. If
  /// no [SharedArrayBufferStore] is specified, SharedArrayBuffer can not be
  /// serialized.
  pub shared_array_buffer_store: Option<SharedArrayBufferStore>,
  pub stdio: Stdio,
}

impl Default for DenoInitOptions {
  fn default() -> Self {
    let cpu_count = thread::available_parallelism()
      .map(|p| p.get())
      .unwrap_or(1);

    let runtime_version = "1.42.1";
    let user_agent = format!("Deno/{runtime_version}");

    Self {
      cwd: std::env::current_dir().unwrap(),
      main_module: None,
      user_agent,
      cpu_count,
      no_color: !colors::use_color(),
      enable_op_summary_metrics: Default::default(),
      locale: "en".to_string(),
      location: Default::default(),
      inspect: Default::default(),
      args: Default::default(),
      argv0: None,
      skip_op_registration: false,
      unsafely_ignore_certificate_errors: Default::default(),
      should_break_on_first_statement: Default::default(),
      should_wait_for_inspector_session: Default::default(),
      shared_array_buffer_store: Default::default(),
      extensions: Default::default(),
      create_params: Default::default(),
      stdio: Default::default(),
    }
  }
}