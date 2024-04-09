// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

pub mod deno_json;
mod flags;
mod flags_net;
mod import_map;
mod lockfile;
pub mod package_json;

pub use self::import_map::resolve_import_map;
use self::package_json::PackageJsonDeps;
use ::import_map::ImportMap;
use deno_core::resolve_url_or_path;
use deno_npm::resolution::ValidSerializedNpmResolutionSnapshot;
use deno_npm::NpmSystemInfo;
use deno_runtime::deno_tls::RootCertStoreProvider;
use deno_semver::npm::NpmPackageReqReference;
use indexmap::IndexMap;

pub use deno_config::glob::FilePatterns;
pub use deno_config::BenchConfig;
pub use deno_config::ConfigFile;
pub use deno_config::FmtOptionsConfig;
pub use deno_config::JsxImportSourceConfig;
pub use deno_config::LintRulesConfig;
pub use deno_config::ProseWrap;
pub use deno_config::TsConfig;
pub use deno_config::TsConfigForEmit;
pub use deno_config::TsConfigType;
pub use deno_config::TsTypeLib;
pub use deno_config::WorkspaceConfig;
pub use flags::*;
pub use lockfile::Lockfile;
pub use lockfile::LockfileError;
pub use package_json::PackageJsonDepsProvider;

use deno_ast::ModuleSpecifier;
use deno_core::anyhow::bail;
use deno_core::anyhow::Context;
use deno_core::error::AnyError;
use deno_core::parking_lot::Mutex;
use deno_core::serde_json;
use deno_core::url::Url;
use deno_runtime::deno_node::PackageJson;
use deno_runtime::deno_tls::deno_native_certs::load_native_certs;
use deno_runtime::deno_tls::rustls;
use deno_runtime::deno_tls::rustls::RootCertStore;
use deno_runtime::deno_tls::rustls_pemfile;
use deno_runtime::deno_tls::webpki_roots;
use deno_runtime::inspector_server::InspectorServer;
use deno_runtime::permissions::PermissionsOptions;
use deno_terminal::colors;
use dotenvy::from_filename;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::io::BufReader;
use std::io::Cursor;
use std::net::SocketAddr;
use std::num::NonZeroUsize;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;

use crate::deno_cli::file_fetcher::FileFetcher;
use crate::deno_cli::util::fs::canonicalize_path_maybe_not_exists;
use crate::deno_cli::version;

use deno_config::glob::PathOrPatternSet;
use deno_config::FmtConfig;
use deno_config::LintConfig;
use deno_config::TestConfig;

pub fn npm_registry_url() -> &'static Url {
  static NPM_REGISTRY_DEFAULT_URL: Lazy<Url> = Lazy::new(|| {
    let env_var_name = "NPM_CONFIG_REGISTRY";
    if let Ok(registry_url) = std::env::var(env_var_name) {
      // ensure there is a trailing slash for the directory
      let registry_url = format!("{}/", registry_url.trim_end_matches('/'));
      match Url::parse(&registry_url) {
        Ok(url) => {
          return url;
        }
        Err(err) => {
          log::debug!("Invalid {} environment variable: {:#}", env_var_name, err,);
        }
      }
    }

    Url::parse("https://registry.npmjs.org").unwrap()
  });

  &NPM_REGISTRY_DEFAULT_URL
}

pub static DENO_DISABLE_PEDANTIC_NODE_WARNINGS: Lazy<bool> = Lazy::new(|| {
  std::env::var("DENO_DISABLE_PEDANTIC_NODE_WARNINGS")
    .ok()
    .is_some()
});

pub fn jsr_url() -> &'static Url {
  static JSR_URL: Lazy<Url> = Lazy::new(|| {
    let env_var_name = "JSR_URL";
    if let Ok(registry_url) = std::env::var(env_var_name) {
      // ensure there is a trailing slash for the directory
      let registry_url = format!("{}/", registry_url.trim_end_matches('/'));
      match Url::parse(&registry_url) {
        Ok(url) => {
          return url;
        }
        Err(err) => {
          log::debug!("Invalid {} environment variable: {:#}", env_var_name, err,);
        }
      }
    }

    Url::parse("https://jsr.io/").unwrap()
  });

  &JSR_URL
}

pub fn jsr_api_url() -> &'static Url {
  static JSR_API_URL: Lazy<Url> = Lazy::new(|| {
    let mut jsr_api_url = jsr_url().clone();
    jsr_api_url.set_path("api/");
    jsr_api_url
  });

  &JSR_API_URL
}

pub fn ts_config_to_emit_options(config: deno_config::TsConfig) -> deno_ast::EmitOptions {
  let options: deno_config::EmitConfigOptions = serde_json::from_value(config.0).unwrap();
  let imports_not_used_as_values = match options.imports_not_used_as_values.as_str() {
    "preserve" => deno_ast::ImportsNotUsedAsValues::Preserve,
    "error" => deno_ast::ImportsNotUsedAsValues::Error,
    _ => deno_ast::ImportsNotUsedAsValues::Remove,
  };
  let (transform_jsx, jsx_automatic, jsx_development, precompile_jsx) = match options.jsx.as_str() {
    "react" => (true, false, false, false),
    "react-jsx" => (true, true, false, false),
    "react-jsxdev" => (true, true, true, false),
    "precompile" => (false, false, false, true),
    _ => (false, false, false, false),
  };
  deno_ast::EmitOptions {
    use_ts_decorators: options.experimental_decorators,
    use_decorators_proposal: !options.experimental_decorators,
    emit_metadata: options.emit_decorator_metadata,
    imports_not_used_as_values,
    inline_source_map: options.inline_source_map,
    inline_sources: options.inline_sources,
    source_map: options.source_map,
    jsx_automatic,
    jsx_development,
    jsx_factory: options.jsx_factory,
    jsx_fragment_factory: options.jsx_fragment_factory,
    jsx_import_source: options.jsx_import_source,
    precompile_jsx,
    transform_jsx,
    var_decl_imports: false,
  }
}

/// Indicates how cached source files should be handled.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CacheSetting {
  /// Only the cached files should be used.  Any files not in the cache will
  /// error.  This is the equivalent of `--cached-only` in the CLI.
  Only,
  /// No cached source files should be used, and all files should be reloaded.
  /// This is the equivalent of `--reload` in the CLI.
  ReloadAll,
  /// Only some cached resources should be used.  This is the equivalent of
  /// `--reload=https://deno.land/std` or
  /// `--reload=https://deno.land/std,https://deno.land/x/example`.
  ReloadSome(Vec<String>),
  /// The usability of a cached value is determined by analyzing the cached
  /// headers and other metadata associated with a cached response, reloading
  /// any cached "non-fresh" cached responses.
  RespectHeaders,
  /// The cached source files should be used for local modules.  This is the
  /// default behavior of the CLI.
  Use,
}

impl CacheSetting {
  pub fn should_use_for_npm_package(
    &self,
    package_name: &str,
  ) -> bool {
    match self {
      CacheSetting::ReloadAll => false,
      CacheSetting::ReloadSome(list) => {
        if list.iter().any(|i| i == "npm:") {
          return false;
        }
        let specifier = format!("npm:{package_name}");
        if list.contains(&specifier) {
          return false;
        }
        true
      }
      _ => true,
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BenchOptions {
  pub files: FilePatterns,
  pub filter: Option<String>,
  pub json: bool,
  pub no_run: bool,
}

impl BenchOptions {
  pub fn resolve(
    maybe_bench_config: Option<BenchConfig>,
    maybe_bench_flags: Option<BenchFlags>,
    initial_cwd: &Path,
  ) -> Result<Self, AnyError> {
    let bench_flags = maybe_bench_flags.unwrap_or_default();
    Ok(Self {
      files: resolve_files(
        maybe_bench_config.map(|c| c.files),
        Some(bench_flags.files),
        initial_cwd,
      )?,
      filter: bench_flags.filter,
      json: bench_flags.json,
      no_run: bench_flags.no_run,
    })
  }
}

#[derive(Clone, Debug)]
pub struct FmtOptions {
  pub check: bool,
  pub options: FmtOptionsConfig,
  pub files: FilePatterns,
}

impl FmtOptions {
  pub fn new_with_base(base: PathBuf) -> Self {
    Self {
      check: false,
      options: FmtOptionsConfig::default(),
      files: FilePatterns::new_with_base(base),
    }
  }

  pub fn resolve(
    maybe_fmt_config: Option<FmtConfig>,
    maybe_fmt_flags: Option<FmtFlags>,
    initial_cwd: &Path,
  ) -> Result<Self, AnyError> {
    let (maybe_config_options, maybe_config_files) =
      maybe_fmt_config.map(|c| (c.options, c.files)).unzip();

    Ok(Self {
      check: maybe_fmt_flags.as_ref().map(|f| f.check).unwrap_or(false),
      options: resolve_fmt_options(maybe_fmt_flags.as_ref(), maybe_config_options),
      files: resolve_files(
        maybe_config_files,
        maybe_fmt_flags.map(|f| f.files),
        initial_cwd,
      )?,
    })
  }
}

fn resolve_fmt_options(
  fmt_flags: Option<&FmtFlags>,
  options: Option<FmtOptionsConfig>,
) -> FmtOptionsConfig {
  let mut options = options.unwrap_or_default();

  if let Some(fmt_flags) = fmt_flags {
    if let Some(use_tabs) = fmt_flags.use_tabs {
      options.use_tabs = Some(use_tabs);
    }

    if let Some(line_width) = fmt_flags.line_width {
      options.line_width = Some(line_width.get());
    }

    if let Some(indent_width) = fmt_flags.indent_width {
      options.indent_width = Some(indent_width.get());
    }

    if let Some(single_quote) = fmt_flags.single_quote {
      options.single_quote = Some(single_quote);
    }

    if let Some(prose_wrap) = &fmt_flags.prose_wrap {
      options.prose_wrap = Some(match prose_wrap.as_str() {
        "always" => ProseWrap::Always,
        "never" => ProseWrap::Never,
        "preserve" => ProseWrap::Preserve,
        // validators in `flags.rs` makes other values unreachable
        _ => unreachable!(),
      });
    }

    if let Some(no_semis) = &fmt_flags.no_semicolons {
      options.semi_colons = Some(!no_semis);
    }
  }

  options
}

#[derive(Clone)]
pub struct TestOptions {
  pub files: FilePatterns,
  pub doc: bool,
  pub no_run: bool,
  pub fail_fast: Option<NonZeroUsize>,
  pub allow_none: bool,
  pub filter: Option<String>,
  pub shuffle: Option<u64>,
  pub concurrent_jobs: NonZeroUsize,
  pub trace_leaks: bool,
  pub reporter: TestReporterConfig,
  pub junit_path: Option<String>,
}

impl TestOptions {
  pub fn resolve(
    maybe_test_config: Option<TestConfig>,
    maybe_test_flags: Option<TestFlags>,
    initial_cwd: &Path,
  ) -> Result<Self, AnyError> {
    let test_flags = maybe_test_flags.unwrap_or_default();

    Ok(Self {
      files: resolve_files(
        maybe_test_config.map(|c| c.files),
        Some(test_flags.files),
        initial_cwd,
      )?,
      allow_none: test_flags.allow_none,
      concurrent_jobs: test_flags
        .concurrent_jobs
        .unwrap_or_else(|| NonZeroUsize::new(1).unwrap()),
      doc: test_flags.doc,
      fail_fast: test_flags.fail_fast,
      filter: test_flags.filter,
      no_run: test_flags.no_run,
      shuffle: test_flags.shuffle,
      trace_leaks: test_flags.trace_leaks,
      reporter: test_flags.reporter,
      junit_path: test_flags.junit_path,
    })
  }
}

#[derive(Clone, Default, Debug)]
pub enum LintReporterKind {
  #[default]
  Pretty,
  Json,
  Compact,
}

#[derive(Clone, Debug)]
pub struct LintOptions {
  pub rules: LintRulesConfig,
  pub files: FilePatterns,
  pub reporter_kind: LintReporterKind,
}

impl LintOptions {
  pub fn new_with_base(base: PathBuf) -> Self {
    Self {
      rules: Default::default(),
      files: FilePatterns::new_with_base(base),
      reporter_kind: Default::default(),
    }
  }

  pub fn resolve(
    maybe_lint_config: Option<LintConfig>,
    maybe_lint_flags: Option<LintFlags>,
    initial_cwd: &Path,
  ) -> Result<Self, AnyError> {
    let mut maybe_reporter_kind = maybe_lint_flags.as_ref().and_then(|lint_flags| {
      if lint_flags.json {
        Some(LintReporterKind::Json)
      } else if lint_flags.compact {
        Some(LintReporterKind::Compact)
      } else {
        None
      }
    });

    if maybe_reporter_kind.is_none() {
      // Flag not set, so try to get lint reporter from the config file.
      if let Some(lint_config) = &maybe_lint_config {
        maybe_reporter_kind = match lint_config.report.as_deref() {
          Some("json") => Some(LintReporterKind::Json),
          Some("compact") => Some(LintReporterKind::Compact),
          Some("pretty") => Some(LintReporterKind::Pretty),
          Some(_) => {
            bail!("Invalid lint report type in config file")
          }
          None => None,
        }
      }
    }

    let (maybe_file_flags, maybe_rules_tags, maybe_rules_include, maybe_rules_exclude) =
      maybe_lint_flags
        .map(|f| {
          (
            f.files,
            f.maybe_rules_tags,
            f.maybe_rules_include,
            f.maybe_rules_exclude,
          )
        })
        .unwrap_or_default();

    let (maybe_config_files, maybe_config_rules) =
      maybe_lint_config.map(|c| (c.files, c.rules)).unzip();
    Ok(Self {
      reporter_kind: maybe_reporter_kind.unwrap_or_default(),
      files: resolve_files(maybe_config_files, Some(maybe_file_flags), initial_cwd)?,
      rules: resolve_lint_rules_options(
        maybe_config_rules,
        maybe_rules_tags,
        maybe_rules_include,
        maybe_rules_exclude,
      ),
    })
  }
}

fn resolve_lint_rules_options(
  maybe_lint_rules_config: Option<LintRulesConfig>,
  mut maybe_rules_tags: Option<Vec<String>>,
  mut maybe_rules_include: Option<Vec<String>>,
  mut maybe_rules_exclude: Option<Vec<String>>,
) -> LintRulesConfig {
  if let Some(config_rules) = maybe_lint_rules_config {
    // Try to get configured rules. CLI flags take precedence
    // over config file, i.e. if there's `rules.include` in config file
    // and `--rules-include` CLI flag, only the flag value is taken into account.
    if maybe_rules_include.is_none() {
      maybe_rules_include = config_rules.include;
    }
    if maybe_rules_exclude.is_none() {
      maybe_rules_exclude = config_rules.exclude;
    }
    if maybe_rules_tags.is_none() {
      maybe_rules_tags = config_rules.tags;
    }
  }
  LintRulesConfig {
    exclude: maybe_rules_exclude,
    include: maybe_rules_include,
    tags: maybe_rules_tags,
  }
}

/// Discover `package.json` file. If `maybe_stop_at` is provided, we will stop
/// crawling up the directory tree at that path.
fn discover_package_json(
  flags: &Flags,
  maybe_stop_at: Option<PathBuf>,
  current_dir: &Path,
) -> Result<Option<PackageJson>, AnyError> {
  // TODO(bartlomieju): discover for all subcommands, but print warnings that
  // `package.json` is ignored in bundle/compile/etc.

  if let Some(package_json_dir) = flags.package_json_search_dir(current_dir) {
    return package_json::discover_from(&package_json_dir, maybe_stop_at);
  }

  log::debug!("No package.json file found");
  Ok(None)
}

struct CliRootCertStoreProvider {
  cell: OnceCell<RootCertStore>,
  maybe_root_path: Option<PathBuf>,
  maybe_ca_stores: Option<Vec<String>>,
  maybe_ca_data: Option<CaData>,
}

impl CliRootCertStoreProvider {
  pub fn new(
    maybe_root_path: Option<PathBuf>,
    maybe_ca_stores: Option<Vec<String>>,
    maybe_ca_data: Option<CaData>,
  ) -> Self {
    Self {
      cell: Default::default(),
      maybe_root_path,
      maybe_ca_stores,
      maybe_ca_data,
    }
  }
}

impl RootCertStoreProvider for CliRootCertStoreProvider {
  fn get_or_try_init(&self) -> Result<&RootCertStore, AnyError> {
    self
      .cell
      .get_or_try_init(|| {
        get_root_cert_store(
          self.maybe_root_path.clone(),
          self.maybe_ca_stores.clone(),
          self.maybe_ca_data.clone(),
        )
      })
      .map_err(|e| e.into())
  }
}

#[derive(Error, Debug, Clone)]
pub enum RootCertStoreLoadError {
  #[error("Unknown certificate store \"{0}\" specified (allowed: \"system,mozilla\")")]
  UnknownStore(String),
  #[error("Unable to add pem file to certificate store: {0}")]
  FailedAddPemFile(String),
  #[error("Failed opening CA file: {0}")]
  CaFileOpenError(String),
}

/// Create and populate a root cert store based on the passed options and
/// environment.
pub fn get_root_cert_store(
  maybe_root_path: Option<PathBuf>,
  maybe_ca_stores: Option<Vec<String>>,
  maybe_ca_data: Option<CaData>,
) -> Result<RootCertStore, RootCertStoreLoadError> {
  let mut root_cert_store = RootCertStore::empty();
  let ca_stores: Vec<String> = maybe_ca_stores
    .or_else(|| {
      let env_ca_store = env::var("DENO_TLS_CA_STORE").ok()?;
      Some(
        env_ca_store
          .split(',')
          .map(|s| s.trim().to_string())
          .filter(|s| !s.is_empty())
          .collect(),
      )
    })
    .unwrap_or_else(|| vec!["mozilla".to_string()]);

  for store in ca_stores.iter() {
    match store.as_str() {
      "mozilla" => {
        root_cert_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
          rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
          )
        }));
      }
      "system" => {
        let roots = load_native_certs().expect("could not load platform certs");
        for root in roots {
          root_cert_store
            .add(&rustls::Certificate(root.0))
            .expect("Failed to add platform cert to root cert store");
        }
      }
      _ => {
        return Err(RootCertStoreLoadError::UnknownStore(store.clone()));
      }
    }
  }

  let ca_data = maybe_ca_data.or_else(|| env::var("DENO_CERT").ok().map(CaData::File));
  if let Some(ca_data) = ca_data {
    let result = match ca_data {
      CaData::File(ca_file) => {
        let ca_file = if let Some(root) = &maybe_root_path {
          root.join(&ca_file)
        } else {
          PathBuf::from(ca_file)
        };
        let certfile = std::fs::File::open(ca_file)
          .map_err(|err| RootCertStoreLoadError::CaFileOpenError(err.to_string()))?;
        let mut reader = BufReader::new(certfile);
        rustls_pemfile::certs(&mut reader)
      }
      CaData::Bytes(data) => {
        let mut reader = BufReader::new(Cursor::new(data));
        rustls_pemfile::certs(&mut reader)
      }
    };

    match result {
      Ok(certs) => {
        root_cert_store.add_parsable_certificates(&certs);
      }
      Err(e) => {
        return Err(RootCertStoreLoadError::FailedAddPemFile(e.to_string()));
      }
    }
  }

  Ok(root_cert_store)
}

/// State provided to the process via an environment variable.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NpmProcessState {
  pub kind: NpmProcessStateKind,
  pub local_node_modules_path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NpmProcessStateKind {
  Snapshot(deno_npm::resolution::SerializedNpmResolutionSnapshot),
  Byonm,
}

const RESOLUTION_STATE_ENV_VAR_NAME: &str = "DENO_DONT_USE_INTERNAL_NODE_COMPAT_STATE";

static NPM_PROCESS_STATE: Lazy<Option<NpmProcessState>> = Lazy::new(|| {
  let state = std::env::var(RESOLUTION_STATE_ENV_VAR_NAME).ok()?;
  let state: NpmProcessState = serde_json::from_str(&state).ok()?;
  // remove the environment variable so that sub processes
  // that are spawned do not also use this.
  std::env::remove_var(RESOLUTION_STATE_ENV_VAR_NAME);
  Some(state)
});

/// Overrides for the options below that when set will
/// use these values over the values derived from the
/// CLI flags or config file.
#[derive(Default, Clone)]
struct CliOptionOverrides {
  import_map_specifier: Option<Option<ModuleSpecifier>>,
}

/// Holds the resolved options of many sources used by subcommands
/// and provides some helper function for creating common objects.
pub struct CliOptions {
  // the source of the options is a detail the rest of the
  // application need not concern itself with, so keep these private
  flags: Flags,
  initial_cwd: PathBuf,
  maybe_node_modules_folder: Option<PathBuf>,
  maybe_vendor_folder: Option<PathBuf>,
  maybe_config_file: Option<ConfigFile>,
  maybe_package_json: Option<PackageJson>,
  maybe_lockfile: Option<Arc<Mutex<Lockfile>>>,
  overrides: CliOptionOverrides,
  maybe_workspace_config: Option<WorkspaceConfig>,
  pub disable_deprecated_api_warning: bool,
  pub verbose_deprecated_api_warning: bool,
}

impl CliOptions {
  pub fn new(
    flags: Flags,
    initial_cwd: PathBuf,
    maybe_config_file: Option<ConfigFile>,
    maybe_lockfile: Option<Arc<Mutex<Lockfile>>>,
    maybe_package_json: Option<PackageJson>,
    force_global_cache: bool,
  ) -> Result<Self, AnyError> {
    if let Some(insecure_allowlist) = flags.unsafely_ignore_certificate_errors.as_ref() {
      let domains = if insecure_allowlist.is_empty() {
        "for all hostnames".to_string()
      } else {
        format!("for: {}", insecure_allowlist.join(", "))
      };
      let msg = format!("DANGER: TLS certificate validation is disabled {domains}");
      // use eprintln instead of log::warn so this always gets shown
      eprintln!("{}", colors::yellow(msg));
    }

    let maybe_lockfile = maybe_lockfile.filter(|_| !force_global_cache);
    let maybe_node_modules_folder = resolve_node_modules_folder(
      &initial_cwd,
      &flags,
      maybe_config_file.as_ref(),
      maybe_package_json.as_ref(),
    )
    .with_context(|| "Resolving node_modules folder.")?;
    let maybe_vendor_folder = if force_global_cache {
      None
    } else {
      resolve_vendor_folder(&initial_cwd, &flags, maybe_config_file.as_ref())
    };
    let maybe_workspace_config = if let Some(config_file) = maybe_config_file.as_ref() {
      config_file.to_workspace_config()?
    } else {
      None
    };

    if let Some(env_file_name) = &flags.env_file {
      if (from_filename(env_file_name)).is_err() {
        log::info!(
          "{} The `--env` flag was used, but the dotenv file '{}' was not found.",
          colors::yellow("Warning"),
          env_file_name
        );
      }
    }

    let disable_deprecated_api_warning = flags.log_level == Some(log::Level::Error)
      || std::env::var("DENO_NO_DEPRECATION_WARNINGS").ok().is_some();

    let verbose_deprecated_api_warning = std::env::var("DENO_VERBOSE_WARNINGS").ok().is_some();

    Ok(Self {
      flags,
      initial_cwd,
      maybe_config_file,
      maybe_lockfile,
      maybe_package_json,
      maybe_node_modules_folder,
      maybe_vendor_folder,
      overrides: Default::default(),
      maybe_workspace_config,
      disable_deprecated_api_warning,
      verbose_deprecated_api_warning,
    })
  }

  pub fn from_flags(flags: Flags) -> Result<Self, AnyError> {
    let initial_cwd = std::env::current_dir().with_context(|| "Failed getting cwd.")?;
    let additional_config_file_names = if matches!(flags.subcommand, DenoSubcommand::Publish(..)) {
      Some(vec!["jsr.json", "jsr.jsonc"])
    } else {
      None
    };
    let maybe_config_file = ConfigFile::discover(
      &flags.config_flag,
      flags.config_path_args(&initial_cwd),
      &initial_cwd,
      additional_config_file_names,
    )?;

    let mut maybe_package_json = None;
    if flags.config_flag == deno_config::ConfigFlag::Disabled
      || flags.no_npm
      || has_flag_env_var("DENO_NO_PACKAGE_JSON")
    {
      log::debug!("package.json auto-discovery is disabled")
    } else if let Some(config_file) = &maybe_config_file {
      let specifier = config_file.specifier.clone();
      if specifier.scheme() == "file" {
        let maybe_stop_at = specifier
          .to_file_path()
          .unwrap()
          .parent()
          .map(|p| p.to_path_buf());

        maybe_package_json = discover_package_json(&flags, maybe_stop_at, &initial_cwd)?;
      }
    } else {
      maybe_package_json = discover_package_json(&flags, None, &initial_cwd)?;
    }

    let maybe_lock_file = lockfile::discover(&flags, maybe_config_file.as_ref())?;
    Self::new(
      flags,
      initial_cwd,
      maybe_config_file,
      maybe_lock_file.map(|l| Arc::new(Mutex::new(l))),
      maybe_package_json,
      false,
    )
  }

  #[inline(always)]
  pub fn initial_cwd(&self) -> &Path {
    &self.initial_cwd
  }

  pub fn maybe_config_file_specifier(&self) -> Option<ModuleSpecifier> {
    self.maybe_config_file.as_ref().map(|f| f.specifier.clone())
  }

  pub fn ts_type_lib_window(&self) -> TsTypeLib {
    TsTypeLib::DenoWindow
  }

  pub fn ts_type_lib_worker(&self) -> TsTypeLib {
    TsTypeLib::DenoWorker
  }

  pub fn cache_setting(&self) -> CacheSetting {
    if self.flags.cached_only {
      CacheSetting::Only
    } else if !self.flags.cache_blocklist.is_empty() {
      CacheSetting::ReloadSome(self.flags.cache_blocklist.clone())
    } else if self.flags.reload {
      CacheSetting::ReloadAll
    } else {
      CacheSetting::Use
    }
  }

  pub fn npm_system_info(&self) -> NpmSystemInfo {
    match self.sub_command() {
      DenoSubcommand::Compile(CompileFlags {
        target: Some(target),
        ..
      }) => {
        // the values of NpmSystemInfo align with the possible values for the
        // `arch` and `platform` fields of Node.js' `process` global:
        // https://nodejs.org/api/process.html
        match target.as_str() {
          "aarch64-apple-darwin" => NpmSystemInfo {
            os: "darwin".to_string(),
            cpu: "arm64".to_string(),
          },
          "aarch64-unknown-linux-gnu" => NpmSystemInfo {
            os: "linux".to_string(),
            cpu: "arm64".to_string(),
          },
          "x86_64-apple-darwin" => NpmSystemInfo {
            os: "darwin".to_string(),
            cpu: "x64".to_string(),
          },
          "x86_64-unknown-linux-gnu" => NpmSystemInfo {
            os: "linux".to_string(),
            cpu: "x64".to_string(),
          },
          "x86_64-pc-windows-msvc" => NpmSystemInfo {
            os: "win32".to_string(),
            cpu: "x64".to_string(),
          },
          value => {
            log::warn!(
              concat!(
                "Not implemented npm system info for target '{}'. Using current ",
                "system default. This may impact architecture specific dependencies."
              ),
              value,
            );
            NpmSystemInfo::default()
          }
        }
      }
      _ => NpmSystemInfo::default(),
    }
  }

  /// Resolve the specifier for a specified import map.
  ///
  /// This will NOT include the config file if it
  /// happens to be an import map.
  pub fn resolve_specified_import_map_specifier(
    &self
  ) -> Result<Option<ModuleSpecifier>, AnyError> {
    match self.overrides.import_map_specifier.clone() {
      Some(maybe_path) => Ok(maybe_path),
      None => resolve_import_map_specifier(
        self.flags.import_map_path.as_deref(),
        self.maybe_config_file.as_ref(),
        &self.initial_cwd,
      ),
    }
  }

  pub async fn resolve_import_map(
    &self,
    file_fetcher: &FileFetcher,
  ) -> Result<Option<ImportMap>, AnyError> {
    if let Some(workspace_config) = self.maybe_workspace_config.as_ref() {
      let root_config_file = self.maybe_config_file.as_ref().unwrap();
      let base_import_map_config = ::import_map::ext::ImportMapConfig {
        base_url: root_config_file.specifier.clone(),
        import_map_value: root_config_file.to_import_map_value_from_imports(),
      };
      let children_configs = workspace_config
        .members
        .iter()
        .map(|member| ::import_map::ext::ImportMapConfig {
          base_url: member.config_file.specifier.clone(),
          import_map_value: member.config_file.to_import_map_value_from_imports(),
        })
        .collect();

      let (import_map_url, import_map) =
        ::import_map::ext::create_synthetic_import_map(base_import_map_config, children_configs);
      log::debug!(
        "Workspace config generated this import map {}",
        serde_json::to_string_pretty(&import_map).unwrap()
      );
      let maybe_import_map_result =
        import_map::import_map_from_value(import_map_url, import_map).map(Some);

      return maybe_import_map_result;
    }

    if self
      .overrides
      .import_map_specifier
      .as_ref()
      .map(|s| s.is_none())
      == Some(true)
    {
      // overrode to not use an import map
      return Ok(None);
    }

    let import_map_specifier = self.resolve_specified_import_map_specifier()?;
    resolve_import_map(
      import_map_specifier.as_ref(),
      self.maybe_config_file().as_ref(),
      file_fetcher,
    )
    .await
  }

  pub fn node_ipc_fd(&self) -> Option<i64> {
    let maybe_node_channel_fd = std::env::var("DENO_CHANNEL_FD").ok();
    if let Some(node_channel_fd) = maybe_node_channel_fd {
      // Remove so that child processes don't inherit this environment variable.
      std::env::remove_var("DENO_CHANNEL_FD");
      node_channel_fd.parse::<i64>().ok()
    } else {
      None
    }
  }

  pub fn enable_future_features(&self) -> bool {
    std::env::var("DENO_FUTURE").is_ok()
  }

  pub fn resolve_main_module(&self) -> Result<ModuleSpecifier, AnyError> {
    match &self.flags.subcommand {
      DenoSubcommand::Bundle(bundle_flags) => {
        resolve_url_or_path(&bundle_flags.source_file, self.initial_cwd()).map_err(AnyError::from)
      }
      DenoSubcommand::Compile(compile_flags) => {
        resolve_url_or_path(&compile_flags.source_file, self.initial_cwd()).map_err(AnyError::from)
      }
      DenoSubcommand::Eval(_) => {
        resolve_url_or_path("./$deno$eval", self.initial_cwd()).map_err(AnyError::from)
      }
      DenoSubcommand::Repl(_) => {
        resolve_url_or_path("./$deno$repl.ts", self.initial_cwd()).map_err(AnyError::from)
      }
      DenoSubcommand::Run(run_flags) => {
        if run_flags.is_stdin() {
          std::env::current_dir()
            .context("Unable to get CWD")
            .and_then(|cwd| resolve_url_or_path("./$deno$stdin.ts", &cwd).map_err(AnyError::from))
        } else if run_flags.watch.is_some() {
          resolve_url_or_path(&run_flags.script, self.initial_cwd()).map_err(AnyError::from)
        } else if NpmPackageReqReference::from_str(&run_flags.script).is_ok() {
          ModuleSpecifier::parse(&run_flags.script).map_err(AnyError::from)
        } else {
          resolve_url_or_path(&run_flags.script, self.initial_cwd()).map_err(AnyError::from)
        }
      }
      _ => {
        bail!("No main module.")
      }
    }
  }

  pub fn resolve_file_header_overrides(&self) -> HashMap<ModuleSpecifier, HashMap<String, String>> {
    let maybe_main_specifier = self.resolve_main_module().ok();
    // TODO(Cre3per): This mapping moved to deno_ast with https://github.com/denoland/deno_ast/issues/133 and should be available in deno_ast >= 0.25.0 via `MediaType::from_path(...).as_media_type()`
    let maybe_content_type = self.flags.ext.as_ref().and_then(|el| match el.as_str() {
      "ts" => Some("text/typescript"),
      "tsx" => Some("text/tsx"),
      "js" => Some("text/javascript"),
      "jsx" => Some("text/jsx"),
      _ => None,
    });

    if let (Some(main_specifier), Some(content_type)) = (maybe_main_specifier, maybe_content_type) {
      HashMap::from([(
        main_specifier,
        HashMap::from([("content-type".to_string(), content_type.to_string())]),
      )])
    } else {
      HashMap::default()
    }
  }

  pub fn resolve_npm_resolution_snapshot(
    &self
  ) -> Result<Option<ValidSerializedNpmResolutionSnapshot>, AnyError> {
    if let Some(NpmProcessStateKind::Snapshot(snapshot)) =
      NPM_PROCESS_STATE.as_ref().map(|s| &s.kind)
    {
      // TODO(bartlomieju): remove this clone
      Ok(Some(snapshot.clone().into_valid()?))
    } else {
      Ok(None)
    }
  }

  // If the main module should be treated as being in an npm package.
  // This is triggered via a secret environment variable which is used
  // for functionality like child_process.fork. Users should NOT depend
  // on this functionality.
  pub fn is_npm_main(&self) -> bool {
    NPM_PROCESS_STATE.is_some()
  }

  /// Overrides the import map specifier to use.
  pub fn set_import_map_specifier(
    &mut self,
    path: Option<ModuleSpecifier>,
  ) {
    self.overrides.import_map_specifier = Some(path);
  }

  pub fn has_node_modules_dir(&self) -> bool {
    self.maybe_node_modules_folder.is_some() || self.unstable_byonm()
  }

  pub fn node_modules_dir_path(&self) -> Option<PathBuf> {
    self.maybe_node_modules_folder.clone()
  }

  pub fn with_node_modules_dir_path(
    &self,
    path: PathBuf,
  ) -> Self {
    Self {
      flags: self.flags.clone(),
      initial_cwd: self.initial_cwd.clone(),
      maybe_node_modules_folder: Some(path),
      maybe_vendor_folder: self.maybe_vendor_folder.clone(),
      maybe_config_file: self.maybe_config_file.clone(),
      maybe_package_json: self.maybe_package_json.clone(),
      maybe_lockfile: self.maybe_lockfile.clone(),
      maybe_workspace_config: self.maybe_workspace_config.clone(),
      overrides: self.overrides.clone(),
      disable_deprecated_api_warning: self.disable_deprecated_api_warning,
      verbose_deprecated_api_warning: self.verbose_deprecated_api_warning,
    }
  }

  pub fn node_modules_dir_enablement(&self) -> Option<bool> {
    self.flags.node_modules_dir.or_else(|| {
      self
        .maybe_config_file
        .as_ref()
        .and_then(|c| c.json.node_modules_dir)
    })
  }

  pub fn vendor_dir_path(&self) -> Option<&PathBuf> {
    self.maybe_vendor_folder.as_ref()
  }

  pub fn resolve_root_cert_store_provider(&self) -> Arc<dyn RootCertStoreProvider> {
    Arc::new(CliRootCertStoreProvider::new(
      None,
      self.flags.ca_stores.clone(),
      self.flags.ca_data.clone(),
    ))
  }

  pub fn resolve_ts_config_for_emit(
    &self,
    config_type: TsConfigType,
  ) -> Result<TsConfigForEmit, AnyError> {
    let result = deno_config::get_ts_config_for_emit(config_type, self.maybe_config_file.as_ref());

    match result {
      Ok(mut ts_config_for_emit) => {
        if matches!(self.flags.subcommand, DenoSubcommand::Bundle(..)) {
          // For backwards compatibility, force `experimentalDecorators` setting
          // to true.
          *ts_config_for_emit
            .ts_config
            .0
            .get_mut("experimentalDecorators")
            .unwrap() = serde_json::Value::Bool(true);
        }
        Ok(ts_config_for_emit)
      }
      Err(err) => Err(err),
    }
  }

  pub fn resolve_inspector_server(&self) -> Option<InspectorServer> {
    let maybe_inspect_host = self
      .flags
      .inspect
      .or(self.flags.inspect_brk)
      .or(self.flags.inspect_wait);
    maybe_inspect_host.map(|host| InspectorServer::new(host, version::get_user_agent()))
  }

  pub fn maybe_lockfile(&self) -> Option<Arc<Mutex<Lockfile>>> {
    self.maybe_lockfile.clone()
  }

  pub fn resolve_tasks_config(&self) -> Result<IndexMap<String, String>, AnyError> {
    if let Some(config_file) = &self.maybe_config_file {
      config_file.resolve_tasks_config()
    } else if self.maybe_package_json.is_some() {
      Ok(Default::default())
    } else {
      bail!("No config file found")
    }
  }

  /// Return the JSX import source configuration.
  pub fn to_maybe_jsx_import_source_config(
    &self
  ) -> Result<Option<JsxImportSourceConfig>, AnyError> {
    match self.maybe_config_file.as_ref() {
      Some(config) => config.to_maybe_jsx_import_source_config(),
      None => Ok(None),
    }
  }

  /// Return any imports that should be brought into the scope of the module
  /// graph.
  pub fn to_maybe_imports(&self) -> Result<Vec<deno_graph::ReferrerImports>, AnyError> {
    if let Some(config_file) = &self.maybe_config_file {
      config_file.to_maybe_imports().map(|maybe_imports| {
        maybe_imports
          .into_iter()
          .map(|(referrer, imports)| deno_graph::ReferrerImports { referrer, imports })
          .collect()
      })
    } else {
      Ok(Vec::new())
    }
  }

  pub fn maybe_config_file(&self) -> &Option<ConfigFile> {
    &self.maybe_config_file
  }

  pub fn maybe_workspace_config(&self) -> &Option<WorkspaceConfig> {
    &self.maybe_workspace_config
  }

  pub fn maybe_package_json(&self) -> &Option<PackageJson> {
    &self.maybe_package_json
  }

  pub fn maybe_package_json_deps(&self) -> Option<PackageJsonDeps> {
    if matches!(
      self.flags.subcommand,
      DenoSubcommand::Task(TaskFlags { task: None, .. })
    ) {
      // don't have any package json dependencies for deno task with no args
      None
    } else {
      self
        .maybe_package_json()
        .as_ref()
        .map(package_json::get_local_package_json_version_reqs)
    }
  }

  pub fn resolve_fmt_options(
    &self,
    fmt_flags: FmtFlags,
  ) -> Result<FmtOptions, AnyError> {
    let maybe_fmt_config = if let Some(config_file) = &self.maybe_config_file {
      config_file.to_fmt_config()?
    } else {
      None
    };
    FmtOptions::resolve(maybe_fmt_config, Some(fmt_flags), &self.initial_cwd)
  }

  pub fn resolve_lint_options(
    &self,
    lint_flags: LintFlags,
  ) -> Result<LintOptions, AnyError> {
    let maybe_lint_config = if let Some(config_file) = &self.maybe_config_file {
      config_file.to_lint_config()?
    } else {
      None
    };
    LintOptions::resolve(maybe_lint_config, Some(lint_flags), &self.initial_cwd)
  }

  pub fn resolve_config_excludes(&self) -> Result<PathOrPatternSet, AnyError> {
    let maybe_config_files = if let Some(config_file) = &self.maybe_config_file {
      Some(config_file.to_files_config()?)
    } else {
      None
    };
    Ok(maybe_config_files.map(|f| f.exclude).unwrap_or_default())
  }

  pub fn resolve_test_options(
    &self,
    test_flags: TestFlags,
  ) -> Result<TestOptions, AnyError> {
    let maybe_test_config = if let Some(config_file) = &self.maybe_config_file {
      config_file.to_test_config()?
    } else {
      None
    };
    TestOptions::resolve(maybe_test_config, Some(test_flags), &self.initial_cwd)
  }

  pub fn resolve_bench_options(
    &self,
    bench_flags: BenchFlags,
  ) -> Result<BenchOptions, AnyError> {
    let maybe_bench_config = if let Some(config_file) = &self.maybe_config_file {
      config_file.to_bench_config()?
    } else {
      None
    };
    BenchOptions::resolve(maybe_bench_config, Some(bench_flags), &self.initial_cwd)
  }

  pub fn resolve_deno_graph_workspace_members(
    &self
  ) -> Result<Vec<deno_graph::WorkspaceMember>, AnyError> {
    fn workspace_config_to_workspace_members(
      workspace_config: &deno_config::WorkspaceConfig
    ) -> Result<Vec<deno_graph::WorkspaceMember>, AnyError> {
      workspace_config
        .members
        .iter()
        .map(|member| {
          config_to_workspace_member(&member.config_file).with_context(|| {
            format!(
              "Failed to resolve configuration for '{}' workspace member at '{}'",
              member.member_name,
              member.config_file.specifier.as_str()
            )
          })
        })
        .collect()
    }

    fn config_to_workspace_member(
      config: &ConfigFile
    ) -> Result<deno_graph::WorkspaceMember, AnyError> {
      let nv = deno_semver::package::PackageNv {
        name: match &config.json.name {
          Some(name) => name.clone(),
          None => bail!("Missing 'name' field in config file."),
        },
        version: match &config.json.version {
          Some(name) => deno_semver::Version::parse_standard(name)?,
          None => bail!("Missing 'version' field in config file."),
        },
      };
      Ok(deno_graph::WorkspaceMember {
        base: config.specifier.join("./").unwrap(),
        nv,
        exports: config.to_exports_config()?.into_map(),
      })
    }

    let maybe_workspace_config = self.maybe_workspace_config();
    if let Some(wc) = maybe_workspace_config {
      workspace_config_to_workspace_members(wc)
    } else {
      Ok(
        self
          .maybe_config_file()
          .as_ref()
          .and_then(|c| match config_to_workspace_member(c) {
            Ok(m) => Some(vec![m]),
            Err(e) => {
              log::debug!("Deno config was not a package: {:#}", e);
              None
            }
          })
          .unwrap_or_default(),
      )
    }
  }

  /// Vector of user script CLI arguments.
  pub fn argv(&self) -> &Vec<String> {
    &self.flags.argv
  }

  pub fn ca_data(&self) -> &Option<CaData> {
    &self.flags.ca_data
  }

  pub fn ca_stores(&self) -> &Option<Vec<String>> {
    &self.flags.ca_stores
  }

  pub fn check_js(&self) -> bool {
    self
      .maybe_config_file
      .as_ref()
      .map(|cf| cf.get_check_js())
      .unwrap_or(false)
  }

  pub fn coverage_dir(&self) -> Option<String> {
    match &self.flags.subcommand {
      DenoSubcommand::Test(test) => test
        .coverage_dir
        .as_ref()
        .map(ToOwned::to_owned)
        .or_else(|| env::var("DENO_UNSTABLE_COVERAGE_DIR").ok()),
      _ => None,
    }
  }

  pub fn enable_op_summary_metrics(&self) -> bool {
    self.flags.enable_op_summary_metrics
      || matches!(
        self.flags.subcommand,
        DenoSubcommand::Test(_) | DenoSubcommand::Repl(_) | DenoSubcommand::Jupyter(_)
      )
  }

  pub fn enable_testing_features(&self) -> bool {
    self.flags.enable_testing_features
  }

  pub fn ext_flag(&self) -> &Option<String> {
    &self.flags.ext
  }

  pub fn has_hmr(&self) -> bool {
    if let DenoSubcommand::Run(RunFlags {
      watch: Some(WatchFlagsWithPaths { hmr, .. }),
      ..
    }) = &self.flags.subcommand
    {
      *hmr
    } else {
      false
    }
  }

  /// If the --inspect or --inspect-brk flags are used.
  pub fn is_inspecting(&self) -> bool {
    self.flags.inspect.is_some()
      || self.flags.inspect_brk.is_some()
      || self.flags.inspect_wait.is_some()
  }

  pub fn inspect_brk(&self) -> Option<SocketAddr> {
    self.flags.inspect_brk
  }

  pub fn inspect_wait(&self) -> Option<SocketAddr> {
    self.flags.inspect_wait
  }

  pub fn log_level(&self) -> Option<log::Level> {
    self.flags.log_level
  }

  pub fn is_quiet(&self) -> bool {
    self
      .log_level()
      .map(|l| l == log::Level::Error)
      .unwrap_or(false)
  }

  pub fn location_flag(&self) -> &Option<Url> {
    &self.flags.location
  }

  pub fn maybe_custom_root(&self) -> &Option<PathBuf> {
    &self.flags.cache_path
  }

  pub fn no_prompt(&self) -> bool {
    resolve_no_prompt(&self.flags)
  }

  pub fn no_remote(&self) -> bool {
    self.flags.no_remote
  }

  pub fn no_npm(&self) -> bool {
    self.flags.no_npm
  }

  pub fn no_config(&self) -> bool {
    self.flags.config_flag == deno_config::ConfigFlag::Disabled
  }

  pub fn permissions_options(&self) -> PermissionsOptions {
    PermissionsOptions {
      allow_env: self.flags.allow_env.clone(),
      deny_env: self.flags.deny_env.clone(),
      allow_hrtime: self.flags.allow_hrtime,
      deny_hrtime: self.flags.deny_hrtime,
      allow_net: self.flags.allow_net.clone(),
      deny_net: self.flags.deny_net.clone(),
      allow_ffi: self.flags.allow_ffi.clone(),
      deny_ffi: self.flags.deny_ffi.clone(),
      allow_read: self.flags.allow_read.clone(),
      deny_read: self.flags.deny_read.clone(),
      allow_run: self.flags.allow_run.clone(),
      deny_run: self.flags.deny_run.clone(),
      allow_sys: self.flags.allow_sys.clone(),
      deny_sys: self.flags.deny_sys.clone(),
      allow_write: self.flags.allow_write.clone(),
      deny_write: self.flags.deny_write.clone(),
      prompt: !self.no_prompt(),
    }
  }

  pub fn reload_flag(&self) -> bool {
    self.flags.reload
  }

  pub fn seed(&self) -> Option<u64> {
    self.flags.seed
  }

  pub fn sub_command(&self) -> &DenoSubcommand {
    &self.flags.subcommand
  }

  pub fn strace_ops(&self) -> &Option<Vec<String>> {
    &self.flags.strace_ops
  }

  pub fn take_binary_npm_command_name(&self) -> Option<String> {
    match self.sub_command() {
      DenoSubcommand::Run(flags) => {
        const NPM_CMD_NAME_ENV_VAR_NAME: &str = "DENO_INTERNAL_NPM_CMD_NAME";
        match std::env::var(NPM_CMD_NAME_ENV_VAR_NAME) {
          Ok(var) => {
            // remove the env var so that child sub processes won't pick this up
            std::env::remove_var(NPM_CMD_NAME_ENV_VAR_NAME);
            Some(var)
          }
          Err(_) => NpmPackageReqReference::from_str(&flags.script)
            .ok()
            .map(|req_ref| npm_pkg_req_ref_to_binary_command(&req_ref)),
        }
      }
      _ => None,
    }
  }

  pub fn type_check_mode(&self) -> TypeCheckMode {
    self.flags.type_check_mode
  }

  pub fn unsafely_ignore_certificate_errors(&self) -> &Option<Vec<String>> {
    &self.flags.unsafely_ignore_certificate_errors
  }

  pub fn legacy_unstable_flag(&self) -> bool {
    self.flags.unstable_config.legacy_flag_enabled
  }

  pub fn unstable_bare_node_builtins(&self) -> bool {
    self.flags.unstable_config.bare_node_builtins
      || self
        .maybe_config_file()
        .as_ref()
        .map(|c| c.has_unstable("bare-node-builtins"))
        .unwrap_or(false)
  }

  pub fn unstable_byonm(&self) -> bool {
    self.flags.unstable_config.byonm
      || NPM_PROCESS_STATE
        .as_ref()
        .map(|s| matches!(s.kind, NpmProcessStateKind::Byonm))
        .unwrap_or(false)
      || self
        .maybe_config_file()
        .as_ref()
        .map(|c| c.has_unstable("byonm"))
        .unwrap_or(false)
  }

  pub fn unstable_sloppy_imports(&self) -> bool {
    self.flags.unstable_config.sloppy_imports
      || self
        .maybe_config_file()
        .as_ref()
        .map(|c| c.has_unstable("sloppy-imports"))
        .unwrap_or(false)
  }

  pub fn unstable_features(&self) -> Vec<String> {
    let mut from_config_file = self
      .maybe_config_file()
      .as_ref()
      .map(|c| c.json.unstable.clone())
      .unwrap_or_default();

    from_config_file.extend_from_slice(&self.flags.unstable_config.features);
    from_config_file
  }

  pub fn v8_flags(&self) -> &Vec<String> {
    &self.flags.v8_flags
  }

  pub fn watch_paths(&self) -> Vec<PathBuf> {
    let mut paths = if let DenoSubcommand::Run(RunFlags {
      watch: Some(WatchFlagsWithPaths { paths, .. }),
      ..
    }) = &self.flags.subcommand
    {
      paths.clone()
    } else {
      Vec::with_capacity(2)
    };
    if let Ok(Some(import_map_path)) = self
      .resolve_specified_import_map_specifier()
      .map(|ms| ms.and_then(|ref s| s.to_file_path().ok()))
    {
      paths.push(import_map_path);
    }
    if let Some(specifier) = self.maybe_config_file_specifier() {
      if specifier.scheme() == "file" {
        if let Ok(path) = specifier.to_file_path() {
          paths.push(path);
        }
      }
    }
    paths
  }
}

/// Resolves the path to use for a local node_modules folder.
fn resolve_node_modules_folder(
  cwd: &Path,
  flags: &Flags,
  maybe_config_file: Option<&ConfigFile>,
  maybe_package_json: Option<&PackageJson>,
) -> Result<Option<PathBuf>, AnyError> {
  let use_node_modules_dir = flags
    .node_modules_dir
    .or_else(|| maybe_config_file.and_then(|c| c.json.node_modules_dir))
    .or(flags.vendor)
    .or_else(|| maybe_config_file.and_then(|c| c.json.vendor));
  let path = if use_node_modules_dir == Some(false) {
    return Ok(None);
  } else if let Some(state) = &*NPM_PROCESS_STATE {
    return Ok(state.local_node_modules_path.as_ref().map(PathBuf::from));
  } else if let Some(package_json_path) = maybe_package_json.map(|c| &c.path) {
    // always auto-discover the local_node_modules_folder when a package.json exists
    package_json_path.parent().unwrap().join("node_modules")
  } else if use_node_modules_dir.is_none() {
    return Ok(None);
  } else if let Some(config_path) = maybe_config_file
    .as_ref()
    .and_then(|c| c.specifier.to_file_path().ok())
  {
    config_path.parent().unwrap().join("node_modules")
  } else {
    cwd.join("node_modules")
  };
  Ok(Some(canonicalize_path_maybe_not_exists(&path)?))
}

fn resolve_vendor_folder(
  cwd: &Path,
  flags: &Flags,
  maybe_config_file: Option<&ConfigFile>,
) -> Option<PathBuf> {
  let use_vendor_dir = flags
    .vendor
    .or_else(|| maybe_config_file.and_then(|c| c.json.vendor))
    .unwrap_or(false);
  // Unlike the node_modules directory, there is no need to canonicalize
  // this directory because it's just used as a cache and the resolved
  // specifier is not based on the canonicalized path (unlike the modules
  // in the node_modules folder).
  if !use_vendor_dir {
    None
  } else if let Some(config_path) = maybe_config_file
    .as_ref()
    .and_then(|c| c.specifier.to_file_path().ok())
  {
    Some(config_path.parent().unwrap().join("vendor"))
  } else {
    Some(cwd.join("vendor"))
  }
}

fn resolve_import_map_specifier(
  maybe_import_map_path: Option<&str>,
  maybe_config_file: Option<&ConfigFile>,
  current_dir: &Path,
) -> Result<Option<ModuleSpecifier>, AnyError> {
  if let Some(import_map_path) = maybe_import_map_path {
    if let Some(config_file) = &maybe_config_file {
      if config_file.json.import_map.is_some() {
        log::warn!("{} the configuration file \"{}\" contains an entry for \"importMap\" that is being ignored.", colors::yellow("Warning"), config_file.specifier);
      }
    }
    let specifier = deno_core::resolve_url_or_path(import_map_path, current_dir)
      .with_context(|| format!("Bad URL (\"{import_map_path}\") for import map."))?;
    Ok(Some(specifier))
  } else if let Some(config_file) = &maybe_config_file {
    // if the config file is an import map we prefer to use it, over `importMap` field
    if config_file.is_an_import_map() && config_file.json.import_map.is_some() {
      log::warn!("{} \"importMap\" setting is ignored when \"imports\" or \"scopes\" are specified in the config file.", colors::yellow("Warning"));
    }
    Ok(None)
  } else {
    Ok(None)
  }
}

pub struct StorageKeyResolver(Option<Option<String>>);

impl StorageKeyResolver {
  pub fn from_options(options: &CliOptions) -> Self {
    Self(if let Some(location) = &options.flags.location {
      // if a location is set, then the ascii serialization of the location is
      // used, unless the origin is opaque, and then no storage origin is set, as
      // we can't expect the origin to be reproducible
      let storage_origin = location.origin();
      if storage_origin.is_tuple() {
        Some(Some(storage_origin.ascii_serialization()))
      } else {
        Some(None)
      }
    } else {
      // otherwise we will use the path to the config file or None to
      // fall back to using the main module's path
      options
        .maybe_config_file
        .as_ref()
        .map(|config_file| Some(config_file.specifier.to_string()))
    })
  }

  /// Creates a storage key resolver that will always resolve to being empty.
  pub fn empty() -> Self {
    Self(Some(None))
  }

  /// Resolves the storage key to use based on the current flags, config, or main module.
  pub fn resolve_storage_key(
    &self,
    main_module: &ModuleSpecifier,
  ) -> Option<String> {
    // use the stored value or fall back to using the path of the main module.
    if let Some(maybe_value) = &self.0 {
      maybe_value.clone()
    } else {
      Some(main_module.to_string())
    }
  }
}

/// Collect included and ignored files. CLI flags take precedence
/// over config file, i.e. if there's `files.ignore` in config file
/// and `--ignore` CLI flag, only the flag value is taken into account.
fn resolve_files(
  maybe_files_config: Option<FilePatterns>,
  maybe_file_flags: Option<FileFlags>,
  initial_cwd: &Path,
) -> Result<FilePatterns, AnyError> {
  let mut maybe_files_config =
    maybe_files_config.unwrap_or_else(|| FilePatterns::new_with_base(initial_cwd.to_path_buf()));
  if let Some(file_flags) = maybe_file_flags {
    if !file_flags.include.is_empty() {
      maybe_files_config.include = Some(PathOrPatternSet::from_include_relative_path_or_patterns(
        initial_cwd,
        &file_flags.include,
      )?);
    }
    if !file_flags.ignore.is_empty() {
      maybe_files_config.exclude =
        PathOrPatternSet::from_exclude_relative_path_or_patterns(initial_cwd, &file_flags.ignore)?;
    }
  }
  Ok(maybe_files_config)
}

/// Resolves the no_prompt value based on the cli flags and environment.
pub fn resolve_no_prompt(flags: &Flags) -> bool {
  flags.no_prompt || has_flag_env_var("DENO_NO_PROMPT")
}

pub fn has_flag_env_var(name: &str) -> bool {
  let value = env::var(name);
  matches!(value.as_ref().map(|s| s.as_str()), Ok("1"))
}

pub fn npm_pkg_req_ref_to_binary_command(req_ref: &NpmPackageReqReference) -> String {
  let binary_name = req_ref.sub_path().unwrap_or(req_ref.req().name.as_str());
  binary_name.to_string()
}
