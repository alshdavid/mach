use std::rc::Rc;
use std::sync::Arc;

use deno_ast::ModuleSpecifier;
use deno_core::error::AnyError;
use deno_core::FsModuleLoader;
use deno_node::NodeResolver;
use deno_runtime::fmt_errors::format_js_error;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;

use crate::deno_cli::node::CliNodeCodeTranslator;
use crate::deno_cli::resolver::CjsResolutionStore;
use crate::DenoInitOptions;
use crate::SNAPSHOT;
use crate::deno_cli::errors;
use crate::deno_cli::npm::byonm::create_byonm_npm_resolver;
use crate::deno_cli::npm::byonm::CliNpmResolverByonmCreateOptions;
use crate::deno_cli::resolver::NpmModuleLoader;

pub async fn init_deno(options: DenoInitOptions) -> Result<MainWorker, AnyError> {
  let fs = Arc::new(deno_fs::RealFs);
  let module_loader = Rc::new(FsModuleLoader);
  let permissions = PermissionsContainer::allow_all();

  let byonm_npm_resolver = create_byonm_npm_resolver(CliNpmResolverByonmCreateOptions {
    fs: fs.clone(),
    root_node_modules_dir: options.cwd.clone(),
  });

  let npm_resolver = byonm_npm_resolver.into_npm_resolver();

  let node_resolver = NodeResolver::new(
    fs.clone(), 
    npm_resolver.clone(),
  );

  let cjs_resolution_store = Arc::new(CjsResolutionStore::default());

  let node_code_translator = Arc::new(CliNodeCodeTranslator::default());
  
  let module_loader = NpmModuleLoader::new(
    cjs_resolution_store.clone(),
    node_code_translator.clone(),
    fs.clone(),
    node_resolver.clone(),
  );

  let bootstrap_options = BootstrapOptions {
    args: options.args,
    argv0: options.argv0,
    cpu_count: options.cpu_count,
    enable_op_summary_metrics: options.enable_op_summary_metrics,
    locale: options.locale,
    location: options.location,
    no_color: options.no_color,
    is_tty: false,
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
    npm_resolver: Some(npm_resolver.clone()),
    source_map_getter: Default::default(),
    maybe_inspector_server: Default::default(),
    strace_ops: Default::default(),
    get_error_class_fn: Some(&errors::get_error_class_name),
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
