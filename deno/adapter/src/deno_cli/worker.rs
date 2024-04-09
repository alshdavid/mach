// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;

use deno_ast::ModuleSpecifier;
use deno_core::anyhow::bail;
use deno_core::anyhow::Context;
use deno_core::error::AnyError;
use deno_core::futures::FutureExt;
use deno_core::located_script_name;
use deno_core::parking_lot::Mutex;
use deno_core::url::Url;
use deno_core::v8;
use deno_core::CompiledWasmModuleStore;
use deno_core::Extension;
use deno_core::FeatureChecker;
use deno_core::ModuleId;
use deno_core::ModuleLoader;
use deno_core::PollEventLoopOptions;
use deno_core::SharedArrayBufferStore;
use deno_core::SourceMapGetter;
use deno_lockfile::Lockfile;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_fs;
use deno_runtime::deno_node;
use deno_runtime::deno_node::NodeResolution;
use deno_runtime::deno_node::NodeResolutionMode;
use deno_runtime::deno_node::NodeResolver;
use deno_runtime::deno_tls::RootCertStoreProvider;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::fmt_errors::format_js_error;
use deno_runtime::inspector_server::InspectorServer;
use deno_runtime::ops::worker_host::CreateWebWorkerCb;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::web_worker::WebWorker;
use deno_runtime::web_worker::WebWorkerOptions;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use deno_runtime::WorkerLogLevel;
use deno_semver::npm::NpmPackageReqReference;
use deno_semver::package::PackageReqReference;
use deno_terminal::colors;
use tokio::select;

use crate::deno_cli::args::package_json::PackageJsonDeps;
use crate::deno_cli::args::DenoSubcommand;
use crate::deno_cli::args::StorageKeyResolver;
use crate::deno_cli::errors;
use crate::deno_cli::npm::CliNpmResolver;
use crate::deno_cli::util::checksum;
use crate::deno_cli::util::file_watcher::WatcherCommunicator;
use crate::deno_cli::util::file_watcher::WatcherRestartMode;
use crate::deno_cli::version;

pub trait ModuleLoaderFactory: Send + Sync {
  fn create_for_main(
    &self,
    root_permissions: PermissionsContainer,
    dynamic_permissions: PermissionsContainer,
  ) -> Rc<dyn ModuleLoader>;

  fn create_for_worker(
    &self,
    root_permissions: PermissionsContainer,
    dynamic_permissions: PermissionsContainer,
  ) -> Rc<dyn ModuleLoader>;

  fn create_source_map_getter(&self) -> Option<Rc<dyn SourceMapGetter>>;
}

// todo(dsherret): this is temporary and we should remove this
// once we no longer conditionally initialize the node runtime
pub trait HasNodeSpecifierChecker: Send + Sync {
  fn has_node_specifier(&self) -> bool;
}

#[async_trait::async_trait(?Send)]
pub trait HmrRunner: Send + Sync {
  async fn start(&mut self) -> Result<(), AnyError>;
  async fn stop(&mut self) -> Result<(), AnyError>;
  async fn run(&mut self) -> Result<(), AnyError>;
}

#[async_trait::async_trait(?Send)]
pub trait CoverageCollector: Send + Sync {
  async fn start_collecting(&mut self) -> Result<(), AnyError>;
  async fn stop_collecting(&mut self) -> Result<(), AnyError>;
}

pub type CreateHmrRunnerCb =
  Box<dyn Fn(deno_core::LocalInspectorSession) -> Box<dyn HmrRunner> + Send + Sync>;

pub type CreateCoverageCollectorCb =
  Box<dyn Fn(deno_core::LocalInspectorSession) -> Box<dyn CoverageCollector> + Send + Sync>;

pub struct CliMainWorkerOptions {
  pub argv: Vec<String>,
  pub log_level: WorkerLogLevel,
  pub coverage_dir: Option<String>,
  pub enable_op_summary_metrics: bool,
  pub enable_testing_features: bool,
  pub has_node_modules_dir: bool,
  pub hmr: bool,
  pub inspect_brk: bool,
  pub inspect_wait: bool,
  pub strace_ops: Option<Vec<String>>,
  pub is_inspecting: bool,
  pub is_npm_main: bool,
  pub location: Option<Url>,
  pub argv0: Option<String>,
  pub origin_data_folder_path: Option<PathBuf>,
  pub seed: Option<u64>,
  pub unsafely_ignore_certificate_errors: Option<Vec<String>>,
  pub unstable: bool,
  pub skip_op_registration: bool,
  pub maybe_root_package_json_deps: Option<PackageJsonDeps>,
  pub create_hmr_runner: Option<CreateHmrRunnerCb>,
  pub create_coverage_collector: Option<CreateCoverageCollectorCb>,
}

struct SharedWorkerState {
  options: CliMainWorkerOptions,
  subcommand: DenoSubcommand,
  storage_key_resolver: StorageKeyResolver,
  npm_resolver: Arc<dyn CliNpmResolver>,
  node_resolver: Arc<NodeResolver>,
  blob_store: Arc<BlobStore>,
  broadcast_channel: InMemoryBroadcastChannel,
  shared_array_buffer_store: SharedArrayBufferStore,
  compiled_wasm_module_store: CompiledWasmModuleStore,
  module_loader_factory: Box<dyn ModuleLoaderFactory>,
  root_cert_store_provider: Arc<dyn RootCertStoreProvider>,
  fs: Arc<dyn deno_fs::FileSystem>,
  maybe_file_watcher_communicator: Option<Arc<WatcherCommunicator>>,
  maybe_inspector_server: Option<Arc<InspectorServer>>,
  maybe_lockfile: Option<Arc<Mutex<Lockfile>>>,
  feature_checker: Arc<FeatureChecker>,
  node_ipc: Option<i64>,
  enable_future_features: bool,
  disable_deprecated_api_warning: bool,
  verbose_deprecated_api_warning: bool,
}

impl SharedWorkerState {
  // Currently empty
}

pub struct CliMainWorker {
  main_module: ModuleSpecifier,
  is_main_cjs: bool,
  pub worker: MainWorker,
  shared: Arc<SharedWorkerState>,
}

impl CliMainWorker {
  pub fn into_main_worker(self) -> MainWorker {
    self.worker
  }

  pub async fn setup_repl(&mut self) -> Result<(), AnyError> {
    self.worker.run_event_loop(false).await?;
    Ok(())
  }

  pub async fn run(&mut self) -> Result<i32, AnyError> {
    let mut maybe_coverage_collector = self.maybe_setup_coverage_collector().await?;
    let mut maybe_hmr_runner = self.maybe_setup_hmr_runner().await?;

    log::debug!("main_module {}", self.main_module);

    if self.is_main_cjs {
      deno_node::load_cjs_module(
        &mut self.worker.js_runtime,
        &self.main_module.to_file_path().unwrap().to_string_lossy(),
        true,
        self.shared.options.inspect_brk,
      )?;
    } else {
      self.execute_main_module_possibly_with_npm().await?;
    }

    self.worker.dispatch_load_event(located_script_name!())?;

    loop {
      if let Some(hmr_runner) = maybe_hmr_runner.as_mut() {
        let watcher_communicator = self.shared.maybe_file_watcher_communicator.clone().unwrap();

        let hmr_future = hmr_runner.run().boxed_local();
        let event_loop_future = self.worker.run_event_loop(false).boxed_local();

        let result;
        select! {
          hmr_result = hmr_future => {
            result = hmr_result;
          },
          event_loop_result = event_loop_future => {
            result = event_loop_result;
          }
        }
        if let Err(e) = result {
          watcher_communicator.change_restart_mode(WatcherRestartMode::Automatic);
          return Err(e);
        }
      } else {
        self
          .worker
          .run_event_loop(maybe_coverage_collector.is_none())
          .await?;
      }

      if !self
        .worker
        .dispatch_beforeunload_event(located_script_name!())?
      {
        break;
      }
    }

    self.worker.dispatch_unload_event(located_script_name!())?;

    if let Some(coverage_collector) = maybe_coverage_collector.as_mut() {
      self
        .worker
        .js_runtime
        .with_event_loop_future(
          coverage_collector.stop_collecting().boxed_local(),
          PollEventLoopOptions::default(),
        )
        .await?;
    }
    if let Some(hmr_runner) = maybe_hmr_runner.as_mut() {
      self
        .worker
        .js_runtime
        .with_event_loop_future(
          hmr_runner.stop().boxed_local(),
          PollEventLoopOptions::default(),
        )
        .await?;
    }

    Ok(self.worker.exit_code())
  }

  pub async fn run_for_watcher(self) -> Result<(), AnyError> {
    /// The FileWatcherModuleExecutor provides module execution with safe dispatching of life-cycle events by tracking the
    /// state of any pending events and emitting accordingly on drop in the case of a future
    /// cancellation.
    struct FileWatcherModuleExecutor {
      inner: CliMainWorker,
      pending_unload: bool,
    }

    impl FileWatcherModuleExecutor {
      pub fn new(worker: CliMainWorker) -> FileWatcherModuleExecutor {
        FileWatcherModuleExecutor {
          inner: worker,
          pending_unload: false,
        }
      }

      /// Execute the given main module emitting load and unload events before and after execution
      /// respectively.
      pub async fn execute(&mut self) -> Result<(), AnyError> {
        self.inner.execute_main_module_possibly_with_npm().await?;
        self
          .inner
          .worker
          .dispatch_load_event(located_script_name!())?;
        self.pending_unload = true;

        let result = loop {
          match self.inner.worker.run_event_loop(false).await {
            Ok(()) => {}
            Err(error) => break Err(error),
          }
          match self
            .inner
            .worker
            .dispatch_beforeunload_event(located_script_name!())
          {
            Ok(default_prevented) if default_prevented => {} // continue loop
            Ok(_) => break Ok(()),
            Err(error) => break Err(error),
          }
        };
        self.pending_unload = false;

        result?;

        self
          .inner
          .worker
          .dispatch_unload_event(located_script_name!())?;

        Ok(())
      }
    }

    impl Drop for FileWatcherModuleExecutor {
      fn drop(&mut self) {
        if self.pending_unload {
          let _ = self
            .inner
            .worker
            .dispatch_unload_event(located_script_name!());
        }
      }
    }

    let mut executor = FileWatcherModuleExecutor::new(self);
    executor.execute().await
  }

  pub async fn execute_main_module_possibly_with_npm(&mut self) -> Result<(), AnyError> {
    let id = self.worker.preload_main_module(&self.main_module).await?;
    self.evaluate_module_possibly_with_npm(id).await
  }

  pub async fn execute_side_module_possibly_with_npm(&mut self) -> Result<(), AnyError> {
    let id = self.worker.preload_side_module(&self.main_module).await?;
    self.evaluate_module_possibly_with_npm(id).await
  }

  async fn evaluate_module_possibly_with_npm(
    &mut self,
    id: ModuleId,
  ) -> Result<(), AnyError> {
    self.worker.evaluate_module(id).await
  }

  pub async fn maybe_setup_hmr_runner(&mut self) -> Result<Option<Box<dyn HmrRunner>>, AnyError> {
    if !self.shared.options.hmr {
      return Ok(None);
    }
    let Some(setup_hmr_runner) = self.shared.options.create_hmr_runner.as_ref() else {
      return Ok(None);
    };

    let session = self.worker.create_inspector_session().await;

    let mut hmr_runner = setup_hmr_runner(session);

    self
      .worker
      .js_runtime
      .with_event_loop_future(
        hmr_runner.start().boxed_local(),
        PollEventLoopOptions::default(),
      )
      .await?;
    Ok(Some(hmr_runner))
  }

  pub async fn maybe_setup_coverage_collector(
    &mut self
  ) -> Result<Option<Box<dyn CoverageCollector>>, AnyError> {
    let Some(create_coverage_collector) = self.shared.options.create_coverage_collector.as_ref()
    else {
      return Ok(None);
    };

    let session = self.worker.create_inspector_session().await;
    let mut coverage_collector = create_coverage_collector(session);
    self
      .worker
      .js_runtime
      .with_event_loop_future(
        coverage_collector.start_collecting().boxed_local(),
        PollEventLoopOptions::default(),
      )
      .await?;
    Ok(Some(coverage_collector))
  }

  pub fn execute_script_static(
    &mut self,
    name: &'static str,
    source_code: &'static str,
  ) -> Result<v8::Global<v8::Value>, AnyError> {
    self.worker.js_runtime.execute_script(name, source_code)
  }
}

pub struct CliMainWorkerFactory {
  shared: Arc<SharedWorkerState>,
}

impl CliMainWorkerFactory {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    storage_key_resolver: StorageKeyResolver,
    subcommand: DenoSubcommand,
    npm_resolver: Arc<dyn CliNpmResolver>,
    node_resolver: Arc<NodeResolver>,
    blob_store: Arc<BlobStore>,
    module_loader_factory: Box<dyn ModuleLoaderFactory>,
    root_cert_store_provider: Arc<dyn RootCertStoreProvider>,
    fs: Arc<dyn deno_fs::FileSystem>,
    maybe_file_watcher_communicator: Option<Arc<WatcherCommunicator>>,
    maybe_inspector_server: Option<Arc<InspectorServer>>,
    maybe_lockfile: Option<Arc<Mutex<Lockfile>>>,
    feature_checker: Arc<FeatureChecker>,
    options: CliMainWorkerOptions,
    node_ipc: Option<i64>,
    enable_future_features: bool,
    disable_deprecated_api_warning: bool,
    verbose_deprecated_api_warning: bool,
  ) -> Self {
    Self {
      shared: Arc::new(SharedWorkerState {
        options,
        subcommand,
        storage_key_resolver,
        npm_resolver,
        node_resolver,
        blob_store,
        broadcast_channel: Default::default(),
        shared_array_buffer_store: Default::default(),
        compiled_wasm_module_store: Default::default(),
        module_loader_factory,
        root_cert_store_provider,
        fs,
        maybe_file_watcher_communicator,
        maybe_inspector_server,
        maybe_lockfile,
        feature_checker,
        node_ipc,
        enable_future_features,
        disable_deprecated_api_warning,
        verbose_deprecated_api_warning,
      }),
    }
  }

  pub async fn create_main_worker(
    &self,
    main_module: ModuleSpecifier,
    permissions: PermissionsContainer,
  ) -> Result<CliMainWorker, AnyError> {
    self
      .create_custom_worker(main_module, permissions, vec![], Default::default())
      .await
  }

  pub async fn create_custom_worker(
    &self,
    main_module: ModuleSpecifier,
    permissions: PermissionsContainer,
    custom_extensions: Vec<Extension>,
    stdio: deno_runtime::deno_io::Stdio,
  ) -> Result<CliMainWorker, AnyError> {
    let shared = &self.shared;
    let (main_module, is_main_cjs) =
      if let Ok(package_ref) = NpmPackageReqReference::from_specifier(&main_module) {
        let package_ref = if package_ref.req().version_req.version_text() == "*" {
          // When using the wildcard version, select the same version used in the
          // package.json deps in order to prevent adding new dependency version
          shared
            .options
            .maybe_root_package_json_deps
            .as_ref()
            .and_then(|deps| {
              deps
                .values()
                .filter_map(|v| v.as_ref().ok())
                .find(|dep| dep.name == package_ref.req().name)
                .map(|dep| {
                  NpmPackageReqReference::new(PackageReqReference {
                    req: dep.clone(),
                    sub_path: package_ref.sub_path().map(|s| s.to_string()),
                  })
                })
            })
            .unwrap_or(package_ref)
        } else {
          package_ref
        };
        if let Some(npm_resolver) = shared.npm_resolver.as_managed() {
          npm_resolver
            .add_package_reqs(&[package_ref.req().clone()])
            .await?;
        }

        // use a fake referrer that can be used to discover the package.json if necessary
        let referrer = ModuleSpecifier::from_directory_path(self.shared.fs.cwd()?)
          .unwrap()
          .join("package.json")?;
        let package_folder = shared
          .npm_resolver
          .resolve_pkg_folder_from_deno_module_req(package_ref.req(), &referrer)?;
        let node_resolution =
          self.resolve_binary_entrypoint(&package_folder, package_ref.sub_path(), &permissions)?;
        let is_main_cjs = matches!(node_resolution, NodeResolution::CommonJs(_));

        if let Some(lockfile) = &shared.maybe_lockfile {
          // For npm binary commands, ensure that the lockfile gets updated
          // so that we can re-use the npm resolution the next time it runs
          // for better performance
          lockfile
            .lock()
            .write()
            .context("Failed writing lockfile.")?;
        }

        (node_resolution.into_url(), is_main_cjs)
      } else if shared.options.is_npm_main || shared.node_resolver.in_npm_package(&main_module) {
        let node_resolution = shared.node_resolver.url_to_node_resolution(main_module)?;
        let is_main_cjs = matches!(node_resolution, NodeResolution::CommonJs(_));
        (node_resolution.into_url(), is_main_cjs)
      } else {
        (main_module, false)
      };

    let module_loader = shared
      .module_loader_factory
      .create_for_main(PermissionsContainer::allow_all(), permissions.clone());
    let maybe_source_map_getter = shared.module_loader_factory.create_source_map_getter();
    let maybe_inspector_server = shared.maybe_inspector_server.clone();

    // let create_web_worker_cb = create_web_worker_callback(shared.clone(), stdio.clone());
    let create_web_worker_cb = create_web_worker_callback(shared.clone(), stdio.clone());
    // let create_web_worker_cb = create_web_worker_callback(shared.clone(), stdio.clone(), custom_extensions.clone());

    let maybe_storage_key = shared
      .storage_key_resolver
      .resolve_storage_key(&main_module);
    let origin_storage_dir = maybe_storage_key.as_ref().map(|key| {
      shared
        .options
        .origin_data_folder_path
        .as_ref()
        .unwrap() // must be set if storage key resolver returns a value
        .join(checksum::gen(&[key.as_bytes()]))
    });
    let cache_storage_dir = maybe_storage_key.map(|key| {
      // TODO(@satyarohith): storage quota management
      // Note: we currently use temp_dir() to avoid managing storage size.
      std::env::temp_dir()
        .join("deno_cache")
        .join(checksum::gen(&[key.as_bytes()]))
    });

    // TODO(bartlomieju): this is cruft, update FeatureChecker to spit out
    // list of enabled features.
    let feature_checker = shared.feature_checker.clone();
    let mut unstable_features = Vec::with_capacity(crate::deno_cli::UNSTABLE_GRANULAR_FLAGS.len());
    for (feature_name, _, id) in crate::deno_cli::UNSTABLE_GRANULAR_FLAGS {
      if feature_checker.check(feature_name) {
        unstable_features.push(*id);
      }
    }

    let options = WorkerOptions {
      bootstrap: BootstrapOptions {
        args: shared.options.argv.clone(),
        cpu_count: std::thread::available_parallelism()
          .map(|p| p.get())
          .unwrap_or(1),
        log_level: shared.options.log_level,
        enable_op_summary_metrics: shared.options.enable_op_summary_metrics,
        enable_testing_features: shared.options.enable_testing_features,
        locale: deno_core::v8::icu::get_language_tag(),
        location: shared.options.location.clone(),
        no_color: !colors::use_color(),
        is_tty: deno_terminal::is_stdout_tty(),
        unstable: shared.options.unstable,
        unstable_features,
        user_agent: version::get_user_agent().to_string(),
        inspect: shared.options.is_inspecting,
        has_node_modules_dir: shared.options.has_node_modules_dir,
        argv0: shared.options.argv0.clone(),
        node_ipc_fd: shared.node_ipc,
        disable_deprecated_api_warning: shared.disable_deprecated_api_warning,
        verbose_deprecated_api_warning: shared.verbose_deprecated_api_warning,
        future: shared.enable_future_features,
      },
      extensions: custom_extensions,
      startup_snapshot: crate::deno_cli::js::deno_isolate_init(),
      create_params: None,
      unsafely_ignore_certificate_errors: shared.options.unsafely_ignore_certificate_errors.clone(),
      root_cert_store_provider: Some(shared.root_cert_store_provider.clone()),
      seed: shared.options.seed,
      source_map_getter: maybe_source_map_getter,
      format_js_error_fn: Some(Arc::new(format_js_error)),
      create_web_worker_cb,
      maybe_inspector_server,
      should_break_on_first_statement: shared.options.inspect_brk,
      should_wait_for_inspector_session: shared.options.inspect_wait,
      strace_ops: shared.options.strace_ops.clone(),
      module_loader,
      fs: shared.fs.clone(),
      npm_resolver: Some(shared.npm_resolver.clone().into_npm_resolver()),
      get_error_class_fn: Some(&errors::get_error_class_name),
      cache_storage_dir,
      origin_storage_dir,
      blob_store: shared.blob_store.clone(),
      broadcast_channel: shared.broadcast_channel.clone(),
      shared_array_buffer_store: Some(shared.shared_array_buffer_store.clone()),
      compiled_wasm_module_store: Some(shared.compiled_wasm_module_store.clone()),
      stdio,
      feature_checker,
      skip_op_registration: false, //shared.options.skip_op_registration,
    };

    let mut worker = MainWorker::bootstrap_from_options(main_module.clone(), permissions, options);

    if self.shared.subcommand.needs_test() {
      macro_rules! test_file {
        ($($file:literal),*) => {
          $(worker.js_runtime.lazy_load_es_module_with_code(
            concat!("ext:cli/", $file),
            deno_core::ascii_str_include!(concat!("js/", $file)),
          )?;)*
        }
      }
      test_file!(
        "40_test_common.js",
        "40_test.js",
        "40_bench.js",
        "40_jupyter.js"
      );
    }

    Ok(CliMainWorker {
      main_module,
      is_main_cjs,
      worker,
      shared: shared.clone(),
    })
  }

  fn resolve_binary_entrypoint(
    &self,
    package_folder: &Path,
    sub_path: Option<&str>,
    permissions: &PermissionsContainer,
  ) -> Result<NodeResolution, AnyError> {
    match self
      .shared
      .node_resolver
      .resolve_binary_export(package_folder, sub_path)
    {
      Ok(node_resolution) => Ok(node_resolution),
      Err(original_err) => {
        // if the binary entrypoint was not found, fallback to regular node resolution
        let result = self.resolve_binary_entrypoint_fallback(package_folder, sub_path, permissions);
        match result {
          Ok(Some(resolution)) => Ok(resolution),
          Ok(None) => Err(original_err),
          Err(fallback_err) => {
            bail!("{:#}\n\nFallback failed: {:#}", original_err, fallback_err)
          }
        }
      }
    }
  }

  /// resolve the binary entrypoint using regular node resolution
  fn resolve_binary_entrypoint_fallback(
    &self,
    package_folder: &Path,
    sub_path: Option<&str>,
    permissions: &PermissionsContainer,
  ) -> Result<Option<NodeResolution>, AnyError> {
    // only fallback if the user specified a sub path
    if sub_path.is_none() {
      // it's confusing to users if the package doesn't have any binary
      // entrypoint and we just execute the main script which will likely
      // have blank output, so do not resolve the entrypoint in this case
      return Ok(None);
    }

    // use a fake referrer since a real one doesn't exist
    let referrer = ModuleSpecifier::from_directory_path(package_folder).unwrap();
    let Some(resolution) = self
      .shared
      .node_resolver
      .resolve_package_subpath_from_deno_module(
        package_folder,
        sub_path,
        &referrer,
        NodeResolutionMode::Execution,
        permissions,
      )?
    else {
      return Ok(None);
    };
    match &resolution {
      NodeResolution::BuiltIn(_) => Ok(None),
      NodeResolution::CommonJs(specifier) | NodeResolution::Esm(specifier) => {
        if specifier
          .to_file_path()
          .map(|p| p.exists())
          .unwrap_or(false)
        {
          Ok(Some(resolution))
        } else {
          bail!("Cannot find module '{}'", specifier)
        }
      }
    }
  }
}

fn create_web_worker_callback(
  shared: Arc<SharedWorkerState>,
  stdio: deno_runtime::deno_io::Stdio,
) -> Arc<CreateWebWorkerCb> {
  Arc::new(move |args| {
    let maybe_inspector_server = shared.maybe_inspector_server.clone();

    let module_loader = shared
      .module_loader_factory
      .create_for_worker(args.parent_permissions.clone(), args.permissions.clone());
    let maybe_source_map_getter = shared.module_loader_factory.create_source_map_getter();
    // let create_web_worker_cb = create_web_worker_callback(shared.clone(), stdio.clone());
    let create_web_worker_cb = create_web_worker_callback(shared.clone(), stdio.clone());

    let maybe_storage_key = shared
      .storage_key_resolver
      .resolve_storage_key(&args.main_module);
    let cache_storage_dir = maybe_storage_key.map(|key| {
      // TODO(@satyarohith): storage quota management
      // Note: we currently use temp_dir() to avoid managing storage size.
      std::env::temp_dir()
        .join("deno_cache")
        .join(checksum::gen(&[key.as_bytes()]))
    });

    // TODO(bartlomieju): this is cruft, update FeatureChecker to spit out
    // list of enabled features.
    let feature_checker = shared.feature_checker.clone();
    let mut unstable_features = Vec::with_capacity(crate::deno_cli::UNSTABLE_GRANULAR_FLAGS.len());
    for (feature_name, _, id) in crate::deno_cli::UNSTABLE_GRANULAR_FLAGS {
      if feature_checker.check(feature_name) {
        unstable_features.push(*id);
      }
    }

    // let custom_extensions = custom_extensions_collection.lock().unwrap().pop().unwrap();

    let options = WebWorkerOptions {
      bootstrap: BootstrapOptions {
        args: shared.options.argv.clone(),
        cpu_count: std::thread::available_parallelism()
          .map(|p| p.get())
          .unwrap_or(1),
        log_level: shared.options.log_level,
        enable_op_summary_metrics: shared.options.enable_op_summary_metrics,
        enable_testing_features: shared.options.enable_testing_features,
        locale: deno_core::v8::icu::get_language_tag(),
        location: Some(args.main_module.clone()),
        no_color: !colors::use_color(),
        is_tty: deno_terminal::is_stdout_tty(),
        unstable: shared.options.unstable,
        unstable_features,
        user_agent: version::get_user_agent().to_string(),
        inspect: shared.options.is_inspecting,
        has_node_modules_dir: shared.options.has_node_modules_dir,
        argv0: shared.options.argv0.clone(),
        node_ipc_fd: None,
        disable_deprecated_api_warning: shared.disable_deprecated_api_warning,
        verbose_deprecated_api_warning: shared.verbose_deprecated_api_warning,
        future: false,
      },
      extensions: vec![],//custom_extensions,
      startup_snapshot: crate::deno_cli::js::deno_isolate_init(),
      unsafely_ignore_certificate_errors: shared.options.unsafely_ignore_certificate_errors.clone(),
      root_cert_store_provider: Some(shared.root_cert_store_provider.clone()),
      seed: shared.options.seed,
      create_web_worker_cb,
      format_js_error_fn: Some(Arc::new(format_js_error)),
      source_map_getter: maybe_source_map_getter,
      module_loader,
      fs: shared.fs.clone(),
      npm_resolver: Some(shared.npm_resolver.clone().into_npm_resolver()),
      worker_type: args.worker_type,
      maybe_inspector_server,
      get_error_class_fn: Some(&errors::get_error_class_name),
      blob_store: shared.blob_store.clone(),
      broadcast_channel: shared.broadcast_channel.clone(),
      shared_array_buffer_store: Some(shared.shared_array_buffer_store.clone()),
      compiled_wasm_module_store: Some(shared.compiled_wasm_module_store.clone()),
      stdio: stdio.clone(),
      cache_storage_dir,
      feature_checker,
    };

    WebWorker::bootstrap_from_options(
      args.name,
      args.permissions,
      args.main_module,
      args.worker_id,
      options,
    )
  })
}
