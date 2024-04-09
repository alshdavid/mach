// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use crate::deno_cli::args::get_root_cert_store;
use crate::deno_cli::args::npm_pkg_req_ref_to_binary_command;
use crate::deno_cli::args::CaData;
use crate::deno_cli::args::CacheSetting;
use crate::deno_cli::args::PackageJsonDepsProvider;
use crate::deno_cli::args::StorageKeyResolver;
use crate::deno_cli::cache::Caches;
use crate::deno_cli::cache::DenoDirProvider;
use crate::deno_cli::cache::NodeAnalysisCache;
use crate::deno_cli::http_util::HttpClient;
use crate::deno_cli::node::CliCjsCodeAnalyzer;
use crate::deno_cli::npm::create_cli_npm_resolver;
use crate::deno_cli::npm::CliNpmResolverByonmCreateOptions;
use crate::deno_cli::npm::CliNpmResolverCreateOptions;
use crate::deno_cli::npm::CliNpmResolverManagedCreateOptions;
use crate::deno_cli::npm::CliNpmResolverManagedPackageJsonInstallerOption;
use crate::deno_cli::npm::CliNpmResolverManagedSnapshotOption;
use crate::deno_cli::npm::NpmCacheDir;
use crate::deno_cli::resolver::CjsResolutionStore;
use crate::deno_cli::resolver::CliNodeResolver;
use crate::deno_cli::resolver::MappedSpecifierResolver;
use crate::deno_cli::resolver::NpmModuleLoader;
use crate::deno_cli::util::progress_bar::ProgressBar;
use crate::deno_cli::util::progress_bar::ProgressBarStyle;
use crate::deno_cli::util::v8::construct_v8_flags;
use crate::deno_cli::worker::CliMainWorkerFactory;
use crate::deno_cli::worker::CliMainWorkerOptions;
use crate::deno_cli::worker::ModuleLoaderFactory;
use deno_ast::MediaType;
use deno_core::anyhow::Context;
use deno_core::error::generic_error;
use deno_core::error::type_error;
use deno_core::error::AnyError;
use deno_core::futures::FutureExt;
use deno_core::v8_set_flags;
use deno_core::FeatureChecker;
use deno_core::ModuleLoader;
use deno_core::ModuleSourceCode;
use deno_core::ModuleSpecifier;
use deno_core::ModuleType;
use deno_core::RequestedModuleType;
use deno_core::ResolutionKind;
use deno_runtime::deno_fs;
use deno_runtime::deno_node::analyze::NodeCodeTranslator;
use deno_runtime::deno_node::NodeResolutionMode;
use deno_runtime::deno_node::NodeResolver;
use deno_runtime::deno_tls::rustls::RootCertStore;
use deno_runtime::deno_tls::RootCertStoreProvider;
use deno_runtime::permissions::Permissions;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::WorkerLogLevel;
use deno_semver::npm::NpmPackageReqReference;
use import_map::parse_from_json;
use std::rc::Rc;
use std::sync::Arc;

pub mod binary;
mod file_system;
mod virtual_fs;

pub use binary::extract_standalone;
pub use binary::is_standalone_binary;
pub use binary::DenoCompileBinaryWriter;

use self::binary::load_npm_vfs;
use self::binary::Metadata;
use self::file_system::DenoCompileFileSystem;

struct SharedModuleLoaderState {
  eszip: eszip::EszipV2,
  mapped_specifier_resolver: MappedSpecifierResolver,
  node_resolver: Arc<CliNodeResolver>,
  npm_module_loader: Arc<NpmModuleLoader>,
}

#[derive(Clone)]
struct EmbeddedModuleLoader {
  shared: Arc<SharedModuleLoaderState>,
  root_permissions: PermissionsContainer,
  dynamic_permissions: PermissionsContainer,
}

impl ModuleLoader for EmbeddedModuleLoader {
  fn resolve(
    &self,
    specifier: &str,
    referrer: &str,
    kind: ResolutionKind,
  ) -> Result<ModuleSpecifier, AnyError> {
    let referrer = if referrer == "." {
      if kind != ResolutionKind::MainModule {
        return Err(generic_error(format!(
          "Expected to resolve main module, got {:?} instead.",
          kind
        )));
      }
      let current_dir = std::env::current_dir().unwrap();
      deno_core::resolve_path(".", &current_dir)?
    } else {
      ModuleSpecifier::parse(referrer)
        .map_err(|err| type_error(format!("Referrer uses invalid specifier: {}", err)))?
    };

    let permissions = if matches!(kind, ResolutionKind::DynamicImport) {
      &self.dynamic_permissions
    } else {
      &self.root_permissions
    };
    if let Some(result) = self.shared.node_resolver.resolve_if_in_npm_package(
      specifier,
      &referrer,
      NodeResolutionMode::Execution,
      permissions,
    ) {
      return match result? {
        Some(res) => Ok(res.into_url()),
        None => Err(generic_error("not found")),
      };
    }

    let maybe_mapped = self
      .shared
      .mapped_specifier_resolver
      .resolve(specifier, &referrer)?
      .into_specifier();

    // npm specifier
    let specifier_text = maybe_mapped
      .as_ref()
      .map(|r| r.as_str())
      .unwrap_or(specifier);
    if let Ok(reference) = NpmPackageReqReference::from_str(specifier_text) {
      return self
        .shared
        .node_resolver
        .resolve_req_reference(
          &reference,
          permissions,
          &referrer,
          NodeResolutionMode::Execution,
        )
        .map(|res| res.into_url());
    }

    let specifier = match maybe_mapped {
      Some(resolved) => resolved,
      None => deno_core::resolve_import(specifier, referrer.as_str())?,
    };
    self
      .shared
      .node_resolver
      .handle_if_in_node_modules(specifier)
  }

  fn load(
    &self,
    original_specifier: &ModuleSpecifier,
    maybe_referrer: Option<&ModuleSpecifier>,
    is_dynamic: bool,
    _requested_module_type: RequestedModuleType,
  ) -> deno_core::ModuleLoadResponse {
    if original_specifier.scheme() == "data" {
      let data_url_text = match deno_graph::source::RawDataUrl::parse(original_specifier)
        .and_then(|url| url.decode().map_err(|err| err.into()))
      {
        Ok(response) => response,
        Err(err) => {
          return deno_core::ModuleLoadResponse::Sync(Err(type_error(format!("{:#}", err))));
        }
      };
      return deno_core::ModuleLoadResponse::Sync(Ok(deno_core::ModuleSource::new(
        deno_core::ModuleType::JavaScript,
        ModuleSourceCode::String(data_url_text.into()),
        original_specifier,
      )));
    }

    let permissions = if is_dynamic {
      &self.dynamic_permissions
    } else {
      &self.root_permissions
    };
    if let Some(result) = self.shared.npm_module_loader.load_sync_if_in_npm_package(
      original_specifier,
      maybe_referrer,
      permissions,
    ) {
      return match result {
        Ok(code_source) => {
          deno_core::ModuleLoadResponse::Sync(Ok(deno_core::ModuleSource::new_with_redirect(
            match code_source.media_type {
              MediaType::Json => ModuleType::Json,
              _ => ModuleType::JavaScript,
            },
            ModuleSourceCode::String(code_source.code),
            original_specifier,
            &code_source.found_url,
          )))
        }
        Err(err) => deno_core::ModuleLoadResponse::Sync(Err(err)),
      };
    }

    let Some(module) = self.shared.eszip.get_module(original_specifier.as_str()) else {
      return deno_core::ModuleLoadResponse::Sync(Err(type_error(format!(
        "Module not found: {}",
        original_specifier
      ))));
    };
    let original_specifier = original_specifier.clone();
    let found_specifier = ModuleSpecifier::parse(&module.specifier).expect("invalid url in eszip");

    deno_core::ModuleLoadResponse::Async(
      async move {
        let code = module
          .source()
          .await
          .ok_or_else(|| type_error(format!("Module not found: {}", original_specifier)))?;
        let code = arc_u8_to_arc_str(code).map_err(|_| type_error("Module source is not utf-8"))?;
        Ok(deno_core::ModuleSource::new_with_redirect(
          match module.kind {
            eszip::ModuleKind::JavaScript => ModuleType::JavaScript,
            eszip::ModuleKind::Json => ModuleType::Json,
            eszip::ModuleKind::Jsonc => return Err(type_error("jsonc modules not supported")),
            eszip::ModuleKind::OpaqueData => {
              unreachable!();
            }
          },
          ModuleSourceCode::String(code.into()),
          &original_specifier,
          &found_specifier,
        ))
      }
      .boxed_local(),
    )
  }
}

fn arc_u8_to_arc_str(arc_u8: Arc<[u8]>) -> Result<Arc<str>, std::str::Utf8Error> {
  // Check that the string is valid UTF-8.
  std::str::from_utf8(&arc_u8)?;
  // SAFETY: the string is valid UTF-8, and the layout Arc<[u8]> is the same as
  // Arc<str>. This is proven by the From<Arc<str>> impl for Arc<[u8]> from the
  // standard library.
  Ok(unsafe { std::mem::transmute(arc_u8) })
}

struct StandaloneModuleLoaderFactory {
  shared: Arc<SharedModuleLoaderState>,
}

impl ModuleLoaderFactory for StandaloneModuleLoaderFactory {
  fn create_for_main(
    &self,
    root_permissions: PermissionsContainer,
    dynamic_permissions: PermissionsContainer,
  ) -> Rc<dyn ModuleLoader> {
    Rc::new(EmbeddedModuleLoader {
      shared: self.shared.clone(),
      root_permissions,
      dynamic_permissions,
    })
  }

  fn create_for_worker(
    &self,
    root_permissions: PermissionsContainer,
    dynamic_permissions: PermissionsContainer,
  ) -> Rc<dyn ModuleLoader> {
    Rc::new(EmbeddedModuleLoader {
      shared: self.shared.clone(),
      root_permissions,
      dynamic_permissions,
    })
  }

  fn create_source_map_getter(&self) -> Option<Rc<dyn deno_core::SourceMapGetter>> {
    None
  }
}

struct StandaloneRootCertStoreProvider {
  ca_stores: Option<Vec<String>>,
  ca_data: Option<CaData>,
  cell: once_cell::sync::OnceCell<RootCertStore>,
}

impl RootCertStoreProvider for StandaloneRootCertStoreProvider {
  fn get_or_try_init(&self) -> Result<&RootCertStore, AnyError> {
    self.cell.get_or_try_init(|| {
      get_root_cert_store(None, self.ca_stores.clone(), self.ca_data.clone())
        .map_err(|err| err.into())
    })
  }
}

pub async fn run(
  mut eszip: eszip::EszipV2,
  metadata: Metadata,
) -> Result<(), AnyError> {
  let main_module = &metadata.entrypoint;
  let current_exe_path = std::env::current_exe().unwrap();
  let current_exe_name = current_exe_path.file_name().unwrap().to_string_lossy();
  let deno_dir_provider = Arc::new(DenoDirProvider::new(None));
  let root_cert_store_provider = Arc::new(StandaloneRootCertStoreProvider {
    ca_stores: metadata.ca_stores,
    ca_data: metadata.ca_data.map(CaData::Bytes),
    cell: Default::default(),
  });
  let progress_bar = ProgressBar::new(ProgressBarStyle::TextOnly);
  let http_client = Arc::new(HttpClient::new(
    Some(root_cert_store_provider.clone()),
    metadata.unsafely_ignore_certificate_errors.clone(),
  ));
  // use a dummy npm registry url
  let npm_registry_url = ModuleSpecifier::parse("https://localhost/").unwrap();
  let root_path = std::env::temp_dir()
    .join(format!("deno-compile-{}", current_exe_name))
    .join("node_modules");
  let npm_cache_dir = NpmCacheDir::new(root_path.clone());
  let npm_global_cache_dir = npm_cache_dir.get_cache_location();
  let cache_setting = CacheSetting::Only;
  let (package_json_deps_provider, fs, npm_resolver, maybe_vfs_root) = match metadata.node_modules {
    Some(binary::NodeModules::Managed {
      node_modules_dir,
      package_json_deps,
    }) => {
      // this will always have a snapshot
      let snapshot = eszip.take_npm_snapshot().unwrap();
      let vfs_root_dir_path = if node_modules_dir {
        root_path
      } else {
        npm_cache_dir.registry_folder(&npm_registry_url)
      };
      let vfs = load_npm_vfs(vfs_root_dir_path.clone()).context("Failed to load npm vfs.")?;
      let maybe_node_modules_path = if node_modules_dir {
        Some(vfs.root().to_path_buf())
      } else {
        None
      };
      let package_json_deps_provider = Arc::new(PackageJsonDepsProvider::new(
        package_json_deps.map(|serialized| serialized.into_deps()),
      ));
      let fs = Arc::new(DenoCompileFileSystem::new(vfs)) as Arc<dyn deno_fs::FileSystem>;
      let npm_resolver = create_cli_npm_resolver(CliNpmResolverCreateOptions::Managed(
        CliNpmResolverManagedCreateOptions {
          snapshot: CliNpmResolverManagedSnapshotOption::Specified(Some(snapshot)),
          maybe_lockfile: None,
          fs: fs.clone(),
          http_client: http_client.clone(),
          npm_global_cache_dir,
          cache_setting,
          text_only_progress_bar: progress_bar,
          maybe_node_modules_path,
          package_json_installer:
            CliNpmResolverManagedPackageJsonInstallerOption::ConditionalInstall(
              package_json_deps_provider.clone(),
            ),
          npm_registry_url,
          npm_system_info: Default::default(),
        },
      ))
      .await?;
      (
        package_json_deps_provider,
        fs,
        npm_resolver,
        Some(vfs_root_dir_path),
      )
    }
    Some(binary::NodeModules::Byonm { package_json_deps }) => {
      let vfs_root_dir_path = root_path;
      let vfs = load_npm_vfs(vfs_root_dir_path.clone()).context("Failed to load npm vfs.")?;
      let node_modules_path = vfs.root().join("node_modules");
      let package_json_deps_provider = Arc::new(PackageJsonDepsProvider::new(
        package_json_deps.map(|serialized| serialized.into_deps()),
      ));
      let fs = Arc::new(DenoCompileFileSystem::new(vfs)) as Arc<dyn deno_fs::FileSystem>;
      let npm_resolver = create_cli_npm_resolver(CliNpmResolverCreateOptions::Byonm(
        CliNpmResolverByonmCreateOptions {
          fs: fs.clone(),
          root_node_modules_dir: node_modules_path,
        },
      ))
      .await?;
      (
        package_json_deps_provider,
        fs,
        npm_resolver,
        Some(vfs_root_dir_path),
      )
    }
    None => {
      let package_json_deps_provider = Arc::new(PackageJsonDepsProvider::new(None));
      let fs = Arc::new(deno_fs::RealFs) as Arc<dyn deno_fs::FileSystem>;
      let npm_resolver = create_cli_npm_resolver(CliNpmResolverCreateOptions::Managed(
        CliNpmResolverManagedCreateOptions {
          snapshot: CliNpmResolverManagedSnapshotOption::Specified(None),
          maybe_lockfile: None,
          fs: fs.clone(),
          http_client: http_client.clone(),
          npm_global_cache_dir,
          cache_setting,
          text_only_progress_bar: progress_bar,
          maybe_node_modules_path: None,
          package_json_installer:
            CliNpmResolverManagedPackageJsonInstallerOption::ConditionalInstall(
              package_json_deps_provider.clone(),
            ),
          npm_registry_url,
          npm_system_info: Default::default(),
        },
      ))
      .await?;
      (package_json_deps_provider, fs, npm_resolver, None)
    }
  };

  let has_node_modules_dir = npm_resolver.root_node_modules_path().is_some();
  let node_resolver = Arc::new(NodeResolver::new(
    fs.clone(),
    npm_resolver.clone().into_npm_resolver(),
  ));
  let cjs_resolutions = Arc::new(CjsResolutionStore::default());
  let cache_db = Caches::new(deno_dir_provider.clone());
  let node_analysis_cache = NodeAnalysisCache::new(cache_db.node_analysis_db());
  let cjs_esm_code_analyzer = CliCjsCodeAnalyzer::new(node_analysis_cache, fs.clone());
  let node_code_translator = Arc::new(NodeCodeTranslator::new(
    cjs_esm_code_analyzer,
    fs.clone(),
    node_resolver.clone(),
    npm_resolver.clone().into_npm_resolver(),
  ));
  let maybe_import_map = metadata
    .maybe_import_map
    .map(|(base, source)| Arc::new(parse_from_json(&base, &source).unwrap().import_map));
  let cli_node_resolver = Arc::new(CliNodeResolver::new(
    Some(cjs_resolutions.clone()),
    fs.clone(),
    node_resolver.clone(),
    npm_resolver.clone(),
  ));
  let module_loader_factory = StandaloneModuleLoaderFactory {
    shared: Arc::new(SharedModuleLoaderState {
      eszip,
      mapped_specifier_resolver: MappedSpecifierResolver::new(
        maybe_import_map.clone(),
        package_json_deps_provider.clone(),
      ),
      node_resolver: cli_node_resolver.clone(),
      npm_module_loader: Arc::new(NpmModuleLoader::new(
        cjs_resolutions,
        node_code_translator,
        fs.clone(),
        cli_node_resolver,
      )),
    }),
  };

  let permissions = {
    let mut permissions = metadata.permissions;
    // if running with an npm vfs, grant read access to it
    if let Some(vfs_root) = maybe_vfs_root {
      match &mut permissions.allow_read {
        Some(vec) if vec.is_empty() => {
          // do nothing, already granted
        }
        Some(vec) => {
          vec.push(vfs_root);
        }
        None => {
          permissions.allow_read = Some(vec![vfs_root]);
        }
      }
    }

    PermissionsContainer::new(Permissions::from_options(&permissions)?)
  };
  let feature_checker = Arc::new({
    let mut checker = FeatureChecker::default();
    checker.set_exit_cb(Box::new(crate::deno_cli::unstable_exit_cb));
    // TODO(bartlomieju): enable, once we deprecate `--unstable` in favor
    // of granular --unstable-* flags.
    // feature_checker.set_warn_cb(Box::new(crate::deno_cli::unstable_warn_cb));
    if metadata.unstable_config.legacy_flag_enabled {
      checker.enable_legacy_unstable();
    }
    for feature in metadata.unstable_config.features {
      // `metadata` is valid for the whole lifetime of the program, so we
      // can leak the string here.
      checker.enable_feature(feature.leak());
    }
    checker
  });
  let worker_factory = CliMainWorkerFactory::new(
    StorageKeyResolver::empty(),
    crate::deno_cli::args::DenoSubcommand::Run(Default::default()),
    npm_resolver,
    node_resolver,
    Default::default(),
    Box::new(module_loader_factory),
    root_cert_store_provider,
    fs,
    None,
    None,
    None,
    feature_checker,
    CliMainWorkerOptions {
      argv: metadata.argv,
      log_level: WorkerLogLevel::Info,
      coverage_dir: None,
      enable_op_summary_metrics: false,
      enable_testing_features: false,
      has_node_modules_dir,
      hmr: false,
      inspect_brk: false,
      inspect_wait: false,
      strace_ops: None,
      is_inspecting: false,
      is_npm_main: main_module.scheme() == "npm",
      skip_op_registration: true,
      location: metadata.location,
      argv0: NpmPackageReqReference::from_specifier(main_module)
        .ok()
        .map(|req_ref| npm_pkg_req_ref_to_binary_command(&req_ref))
        .or(std::env::args().next()),
      origin_data_folder_path: None,
      seed: metadata.seed,
      unsafely_ignore_certificate_errors: metadata.unsafely_ignore_certificate_errors,
      unstable: metadata.unstable_config.legacy_flag_enabled,
      maybe_root_package_json_deps: package_json_deps_provider.deps().cloned(),
      create_hmr_runner: None,
      create_coverage_collector: None,
    },
    None,
    false,
    // TODO(bartlomieju): temporarily disabled
    // metadata.disable_deprecated_api_warning,
    true,
    false,
  );

  v8_set_flags(construct_v8_flags(&[], &metadata.v8_flags, vec![]));

  let mut worker = worker_factory
    .create_main_worker(main_module.clone(), permissions)
    .await?;

  let exit_code = worker.run().await?;
  std::process::exit(exit_code)
}
