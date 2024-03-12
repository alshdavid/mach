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

use crate::deno_cli::args::flags_from_vec;
use crate::deno_cli::args::CliOptions;
use crate::deno_cli::cache::Caches;
use crate::deno_cli::cache::DenoDirProvider;
use crate::deno_cli::cache::NodeAnalysisCache;
use crate::deno_cli::cache::ParsedSourceCache;
use crate::deno_cli::errors;
use crate::deno_cli::module_loader::CliModuleLoader;
use crate::deno_cli::module_loader::CliModuleLoaderFactory;
use crate::deno_cli::node::CliCjsCodeAnalyzer;
use crate::deno_cli::node::CliNodeCodeTranslator;
use crate::deno_cli::npm::byonm::create_byonm_npm_resolver;
use crate::deno_cli::npm::byonm::CliNpmResolverByonmCreateOptions;
use crate::deno_cli::resolver::CjsResolutionStore;
use crate::deno_cli::resolver::CliNodeResolver;
use crate::deno_cli::resolver::NpmModuleLoader;
use crate::DenoInitOptions;
use crate::SNAPSHOT;

pub async fn init_deno(options: DenoInitOptions) -> Result<MainWorker, AnyError> {
  let fs = Arc::new(deno_fs::RealFs);
  let module_loader = Rc::new(FsModuleLoader);
  let permissions = PermissionsContainer::allow_all();

  let byonm_npm_resolver = create_byonm_npm_resolver(CliNpmResolverByonmCreateOptions {
    fs: fs.clone(),
    root_node_modules_dir: options.cwd.clone(),
  });

  let npm_resolver = byonm_npm_resolver.into_npm_resolver();

  let node_resolver = Arc::new(NodeResolver::new(fs.clone(), npm_resolver.clone()));

  let cjs_resolution_store = Arc::new(CjsResolutionStore::default());

  let deno_dir_provider = Arc::new(DenoDirProvider::new(None));

  let caches = Arc::new(Caches::new(deno_dir_provider.clone()));

  let node_analysis_cache = NodeAnalysisCache::new(caches.node_analysis_db());

  let cli_cjs_code_analyzer = CliCjsCodeAnalyzer::new(node_analysis_cache, fs.clone());

  let node_code_translator = Arc::new(CliNodeCodeTranslator::new(
    cli_cjs_code_analyzer,
    fs.clone(),
    node_resolver.clone(),
    npm_resolver.clone(),
  ));

  let cli_node_resolver = Arc::new(CliNodeResolver::new(
    Some(cjs_resolution_store.clone()),
    fs.clone(),
    node_resolver.clone(),
    byonm_npm_resolver.clone(),
  ));

  let npm_module_loader = Rc::new(NpmModuleLoader::new(
    cjs_resolution_store.clone(),
    node_code_translator.clone(),
    fs.clone(),
    cli_node_resolver.clone(),
  ));

  let cli_options = CliOptions::new(
    flags_from_vec(vec![]),
    options.cwd.clone(),
    None,
    None,
    None,
    false,
  );

  let parsed_source_cache = ParsedSourceCache::default();
  
  let emit_cache = EmitCache::new(deno_dir_provider.get_or_create()?.gen_cache.clone());
  let emitter = Arc::new(Emitter::new());

  let cli_module_loader_factory = CliModuleLoaderFactory::new(
    &cli_options,
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
    create_web_worker_cb: Arc::new(|_| unimplemented!("web workers are not supported")),
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

  Ok(main_worker)
}
