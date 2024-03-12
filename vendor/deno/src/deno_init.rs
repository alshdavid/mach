use std::rc::Rc;
use std::sync::Arc;

use deno_ast::ModuleSpecifier;
use deno_core::FsModuleLoader;
use deno_runtime::fmt_errors::format_js_error;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;

use crate::DenoInitOptions;
use crate::SNAPSHOT;

pub fn init_deno(options: DenoInitOptions) -> MainWorker {
  let fs = Arc::new(deno_fs::RealFs);
  let module_loader = Rc::new(FsModuleLoader);
  let permissions = PermissionsContainer::allow_all();

  let bootstrap_options = BootstrapOptions {
    args: options.args,
    argv0: options.argv0,
    cpu_count: options.cpu_count,
    enable_op_summary_metrics: options.enable_op_summary_metrics,
    locale: options.locale,
    location: options.location,
    no_color: options.no_color,
    is_tty: options.is_tty,
    user_agent: options.user_agent,
    inspect: options.inspect,
    log_level: Default::default(),
    enable_testing_features: false,
    unstable: false,
    unstable_features: Default::default(),
    has_node_modules_dir: Default::default(),
    node_ipc_fd: None,
    disable_deprecated_api_warning: false,
    verbose_deprecated_api_warning: false,
    future: false,
  };

  let worker_options = WorkerOptions {
    bootstrap: bootstrap_options.clone(),
    extensions: options.extensions,
    skip_op_registration: options.skip_op_registration,
    create_params: options.create_params,
    unsafely_ignore_certificate_errors: options.unsafely_ignore_certificate_errors,
    should_break_on_first_statement: options.should_break_on_first_statement,
    should_wait_for_inspector_session: options.should_wait_for_inspector_session,
    shared_array_buffer_store: options.shared_array_buffer_store,
    stdio: options.stdio,
    startup_snapshot: Some(SNAPSHOT),
    fs: fs.clone(),
    create_web_worker_cb: Arc::new(|_| {
      unimplemented!("web workers are not supported")
    }),
    format_js_error_fn: Some(Arc::new(format_js_error)),
    root_cert_store_provider: Default::default(),
    seed: Default::default(),
    module_loader: module_loader.clone(),
    npm_resolver: Default::default(),
    source_map_getter: Default::default(),
    maybe_inspector_server: Default::default(),
    strace_ops: Default::default(),
    get_error_class_fn: Default::default(),
    cache_storage_dir: Default::default(),
    origin_storage_dir: Default::default(),
    blob_store: Default::default(),
    broadcast_channel: Default::default(),
    compiled_wasm_module_store: Default::default(),
    feature_checker: Default::default(),
  };

  let main_module = 'block: {
    if let Some(main_module) = &options.main_module {
      break 'block ModuleSpecifier::from_file_path(main_module).unwrap();
    }
    let exe_path = std::env::current_exe().unwrap();
    break 'block ModuleSpecifier::from_file_path(exe_path).unwrap();
  };
  
  let mut main_worker = MainWorker::from_options(main_module.clone(), permissions, worker_options);
  main_worker.bootstrap(bootstrap_options.clone());
  
  main_worker
}
