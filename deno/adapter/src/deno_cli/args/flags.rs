// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use clap::builder::styling::AnsiColor;
use clap::builder::FalseyValueParser;
use clap::value_parser;
use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::ColorChoice;
use clap::Command;
use clap::ValueHint;
use deno_config::ConfigFlag;
use deno_core::resolve_url_or_path;
use deno_core::url::Url;
use deno_graph::GraphKind;
use deno_runtime::permissions::parse_sys_kind;
use log::debug;
use log::Level;
use std::env;
use std::net::SocketAddr;
use std::num::NonZeroU32;
use std::num::NonZeroU8;
use std::num::NonZeroUsize;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use crate::deno_cli::util::fs::canonicalize_path;

use super::flags_net;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FileFlags {
  pub ignore: Vec<String>,
  pub include: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AddFlags {
  pub packages: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BenchFlags {
  pub files: FileFlags,
  pub filter: Option<String>,
  pub json: bool,
  pub no_run: bool,
  pub watch: Option<WatchFlags>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BundleFlags {
  pub source_file: String,
  pub out_file: Option<String>,
  pub watch: Option<WatchFlags>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CacheFlags {
  pub files: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckFlags {
  pub files: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompileFlags {
  pub source_file: String,
  pub output: Option<PathBuf>,
  pub args: Vec<String>,
  pub target: Option<String>,
  pub no_terminal: bool,
  pub include: Vec<String>,
}

impl CompileFlags {
  pub fn resolve_target(&self) -> String {
    self.target.clone().unwrap_or_else(|| "release".to_string())
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletionsFlags {
  pub buf: Box<[u8]>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum CoverageType {
  #[default]
  Summary,
  Detailed,
  Lcov,
  Html,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct CoverageFlags {
  pub files: FileFlags,
  pub output: Option<PathBuf>,
  pub include: Vec<String>,
  pub exclude: Vec<String>,
  pub r#type: CoverageType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DocSourceFileFlag {
  Builtin,
  Paths(Vec<String>),
}

impl Default for DocSourceFileFlag {
  fn default() -> Self {
    Self::Builtin
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocHtmlFlag {
  pub name: String,
  pub output: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocFlags {
  pub private: bool,
  pub json: bool,
  pub lint: bool,
  pub html: Option<DocHtmlFlag>,
  pub source_files: DocSourceFileFlag,
  pub filter: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvalFlags {
  pub print: bool,
  pub code: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FmtFlags {
  pub check: bool,
  pub files: FileFlags,
  pub use_tabs: Option<bool>,
  pub line_width: Option<NonZeroU32>,
  pub indent_width: Option<NonZeroU8>,
  pub single_quote: Option<bool>,
  pub prose_wrap: Option<String>,
  pub no_semicolons: Option<bool>,
  pub watch: Option<WatchFlags>,
}

impl FmtFlags {
  pub fn is_stdin(&self) -> bool {
    let args = &self.files.include;
    args.len() == 1 && args[0] == "-"
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitFlags {
  pub dir: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfoFlags {
  pub json: bool,
  pub file: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstallFlags {
  pub module_url: String,
  pub args: Vec<String>,
  pub name: Option<String>,
  pub root: Option<String>,
  pub force: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JupyterFlags {
  pub install: bool,
  pub kernel: bool,
  pub conn_file: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UninstallFlags {
  pub name: String,
  pub root: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LintFlags {
  pub files: FileFlags,
  pub rules: bool,
  pub maybe_rules_tags: Option<Vec<String>>,
  pub maybe_rules_include: Option<Vec<String>>,
  pub maybe_rules_exclude: Option<Vec<String>>,
  pub json: bool,
  pub compact: bool,
  pub watch: Option<WatchFlags>,
}

impl LintFlags {
  pub fn is_stdin(&self) -> bool {
    let args = &self.files.include;
    args.len() == 1 && args[0] == "-"
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplFlags {
  pub eval_files: Option<Vec<String>>,
  pub eval: Option<String>,
  pub is_default_command: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct RunFlags {
  pub script: String,
  pub watch: Option<WatchFlagsWithPaths>,
}

impl RunFlags {
  pub fn is_stdin(&self) -> bool {
    self.script == "-"
  }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct WatchFlags {
  pub hmr: bool,
  pub no_clear_screen: bool,
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct WatchFlagsWithPaths {
  pub hmr: bool,
  pub paths: Vec<PathBuf>,
  pub no_clear_screen: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskFlags {
  pub cwd: Option<String>,
  pub task: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum TestReporterConfig {
  #[default]
  Pretty,
  Dot,
  Junit,
  Tap,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TestFlags {
  pub doc: bool,
  pub no_run: bool,
  pub coverage_dir: Option<String>,
  pub fail_fast: Option<NonZeroUsize>,
  pub files: FileFlags,
  pub allow_none: bool,
  pub filter: Option<String>,
  pub shuffle: Option<u64>,
  pub concurrent_jobs: Option<NonZeroUsize>,
  pub trace_leaks: bool,
  pub watch: Option<WatchFlags>,
  pub reporter: TestReporterConfig,
  pub junit_path: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpgradeFlags {
  pub dry_run: bool,
  pub force: bool,
  pub canary: bool,
  pub version: Option<String>,
  pub output: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VendorFlags {
  pub specifiers: Vec<String>,
  pub output_path: Option<PathBuf>,
  pub force: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishFlags {
  pub token: Option<String>,
  pub dry_run: bool,
  pub allow_slow_types: bool,
  pub allow_dirty: bool,
  pub no_provenance: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DenoSubcommand {
  Add(AddFlags),
  Bench(BenchFlags),
  Bundle(BundleFlags),
  Cache(CacheFlags),
  Check(CheckFlags),
  Compile(CompileFlags),
  Completions(CompletionsFlags),
  Coverage(CoverageFlags),
  Doc(DocFlags),
  Eval(EvalFlags),
  Fmt(FmtFlags),
  Init(InitFlags),
  Info(InfoFlags),
  Install(InstallFlags),
  Jupyter(JupyterFlags),
  Uninstall(UninstallFlags),
  Lsp,
  Lint(LintFlags),
  Repl(ReplFlags),
  Run(RunFlags),
  Task(TaskFlags),
  Test(TestFlags),
  Types,
  Upgrade(UpgradeFlags),
  Vendor(VendorFlags),
  Publish(PublishFlags),
}

impl DenoSubcommand {
  pub fn is_run(&self) -> bool {
    matches!(self, Self::Run(_))
  }

  // Returns `true` if the subcommand depends on testing infrastructure.
  pub fn needs_test(&self) -> bool {
    matches!(
      self,
      Self::Test(_) | Self::Jupyter(_) | Self::Repl(_) | Self::Bench(_) | Self::Lsp
    )
  }
}

impl Default for DenoSubcommand {
  fn default() -> DenoSubcommand {
    DenoSubcommand::Repl(ReplFlags {
      eval_files: None,
      eval: None,
      is_default_command: true,
    })
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TypeCheckMode {
  /// Type-check all modules.
  All,
  /// Skip type-checking of all modules. The default value for "deno run" and
  /// several other subcommands.
  None,
  /// Only type-check local modules. The default value for "deno test" and
  /// several other subcommands.
  Local,
}

impl TypeCheckMode {
  /// Gets if type checking will occur under this mode.
  pub fn is_true(&self) -> bool {
    match self {
      Self::None => false,
      Self::Local | Self::All => true,
    }
  }

  /// Gets the corresponding module `GraphKind` that should be created
  /// for the current `TypeCheckMode`.
  pub fn as_graph_kind(&self) -> GraphKind {
    match self.is_true() {
      true => GraphKind::All,
      false => GraphKind::CodeOnly,
    }
  }
}

impl Default for TypeCheckMode {
  fn default() -> Self {
    Self::None
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CaData {
  /// The string is a file path
  File(String),
  /// This variant is not exposed as an option in the CLI, it is used internally
  /// for standalone binaries.
  Bytes(Vec<u8>),
}

#[derive(Clone, Default, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnstableConfig {
  pub legacy_flag_enabled: bool, // --unstable
  pub bare_node_builtins: bool,  // --unstable-bare-node-builts
  pub byonm: bool,
  pub sloppy_imports: bool,
  pub features: Vec<String>, // --unstabe-kv --unstable-cron
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Flags {
  /// Vector of CLI arguments - these are user script arguments, all Deno
  /// specific flags are removed.
  pub argv: Vec<String>,
  pub subcommand: DenoSubcommand,

  pub allow_all: bool,
  pub allow_env: Option<Vec<String>>,
  pub deny_env: Option<Vec<String>>,
  pub allow_hrtime: bool,
  pub deny_hrtime: bool,
  pub allow_net: Option<Vec<String>>,
  pub deny_net: Option<Vec<String>>,
  pub allow_ffi: Option<Vec<PathBuf>>,
  pub deny_ffi: Option<Vec<PathBuf>>,
  pub allow_read: Option<Vec<PathBuf>>,
  pub deny_read: Option<Vec<PathBuf>>,
  pub allow_run: Option<Vec<String>>,
  pub deny_run: Option<Vec<String>>,
  pub allow_sys: Option<Vec<String>>,
  pub deny_sys: Option<Vec<String>>,
  pub allow_write: Option<Vec<PathBuf>>,
  pub deny_write: Option<Vec<PathBuf>>,
  pub ca_stores: Option<Vec<String>>,
  pub ca_data: Option<CaData>,
  pub cache_blocklist: Vec<String>,
  /// This is not exposed as an option in the CLI, it is used internally when
  /// the language server is configured with an explicit cache option.
  pub cache_path: Option<PathBuf>,
  pub cached_only: bool,
  pub type_check_mode: TypeCheckMode,
  pub config_flag: ConfigFlag,
  pub node_modules_dir: Option<bool>,
  pub vendor: Option<bool>,
  pub enable_op_summary_metrics: bool,
  pub enable_testing_features: bool,
  pub ext: Option<String>,
  pub ignore: Vec<PathBuf>,
  pub import_map_path: Option<String>,
  pub env_file: Option<String>,
  pub inspect_brk: Option<SocketAddr>,
  pub inspect_wait: Option<SocketAddr>,
  pub inspect: Option<SocketAddr>,
  pub location: Option<Url>,
  pub lock_write: bool,
  pub lock: Option<PathBuf>,
  pub log_level: Option<Level>,
  pub no_remote: bool,
  pub no_lock: bool,
  pub no_npm: bool,
  pub no_prompt: bool,
  pub reload: bool,
  pub seed: Option<u64>,
  pub strace_ops: Option<Vec<String>>,
  pub unstable_config: UnstableConfig,
  pub unsafely_ignore_certificate_errors: Option<Vec<String>>,
  pub v8_flags: Vec<String>,
}

fn join_paths(
  allowlist: &[PathBuf],
  d: &str,
) -> String {
  allowlist
    .iter()
    .map(|path| path.to_str().unwrap().to_string())
    .collect::<Vec<String>>()
    .join(d)
}

impl Flags {
  /// Return list of permission arguments that are equivalent
  /// to the ones used to create `self`.
  pub fn to_permission_args(&self) -> Vec<String> {
    let mut args = vec![];

    if self.allow_all {
      args.push("--allow-all".to_string());
      return args;
    }

    match &self.allow_read {
      Some(read_allowlist) if read_allowlist.is_empty() => {
        args.push("--allow-read".to_string());
      }
      Some(read_allowlist) => {
        let s = format!("--allow-read={}", join_paths(read_allowlist, ","));
        args.push(s);
      }
      _ => {}
    }

    match &self.deny_read {
      Some(read_denylist) if read_denylist.is_empty() => {
        args.push("--deny-read".to_string());
      }
      Some(read_denylist) => {
        let s = format!("--deny-read={}", join_paths(read_denylist, ","));
        args.push(s);
      }
      _ => {}
    }

    match &self.allow_write {
      Some(write_allowlist) if write_allowlist.is_empty() => {
        args.push("--allow-write".to_string());
      }
      Some(write_allowlist) => {
        let s = format!("--allow-write={}", join_paths(write_allowlist, ","));
        args.push(s);
      }
      _ => {}
    }

    match &self.deny_write {
      Some(write_denylist) if write_denylist.is_empty() => {
        args.push("--deny-write".to_string());
      }
      Some(write_denylist) => {
        let s = format!("--deny-write={}", join_paths(write_denylist, ","));
        args.push(s);
      }
      _ => {}
    }

    match &self.allow_net {
      Some(net_allowlist) if net_allowlist.is_empty() => {
        args.push("--allow-net".to_string());
      }
      Some(net_allowlist) => {
        let s = format!("--allow-net={}", net_allowlist.join(","));
        args.push(s);
      }
      _ => {}
    }

    match &self.deny_net {
      Some(net_denylist) if net_denylist.is_empty() => {
        args.push("--deny-net".to_string());
      }
      Some(net_denylist) => {
        let s = format!("--deny-net={}", net_denylist.join(","));
        args.push(s);
      }
      _ => {}
    }

    match &self.unsafely_ignore_certificate_errors {
      Some(ic_allowlist) if ic_allowlist.is_empty() => {
        args.push("--unsafely-ignore-certificate-errors".to_string());
      }
      Some(ic_allowlist) => {
        let s = format!(
          "--unsafely-ignore-certificate-errors={}",
          ic_allowlist.join(",")
        );
        args.push(s);
      }
      _ => {}
    }

    match &self.allow_env {
      Some(env_allowlist) if env_allowlist.is_empty() => {
        args.push("--allow-env".to_string());
      }
      Some(env_allowlist) => {
        let s = format!("--allow-env={}", env_allowlist.join(","));
        args.push(s);
      }
      _ => {}
    }

    match &self.deny_env {
      Some(env_denylist) if env_denylist.is_empty() => {
        args.push("--deny-env".to_string());
      }
      Some(env_denylist) => {
        let s = format!("--deny-env={}", env_denylist.join(","));
        args.push(s);
      }
      _ => {}
    }

    match &self.allow_run {
      Some(run_allowlist) if run_allowlist.is_empty() => {
        args.push("--allow-run".to_string());
      }
      Some(run_allowlist) => {
        let s = format!("--allow-run={}", run_allowlist.join(","));
        args.push(s);
      }
      _ => {}
    }

    match &self.deny_run {
      Some(run_denylist) if run_denylist.is_empty() => {
        args.push("--deny-run".to_string());
      }
      Some(run_denylist) => {
        let s = format!("--deny-run={}", run_denylist.join(","));
        args.push(s);
      }
      _ => {}
    }

    match &self.allow_sys {
      Some(sys_allowlist) if sys_allowlist.is_empty() => {
        args.push("--allow-sys".to_string());
      }
      Some(sys_allowlist) => {
        let s = format!("--allow-sys={}", sys_allowlist.join(","));
        args.push(s)
      }
      _ => {}
    }

    match &self.deny_sys {
      Some(sys_denylist) if sys_denylist.is_empty() => {
        args.push("--deny-sys".to_string());
      }
      Some(sys_denylist) => {
        let s = format!("--deny-sys={}", sys_denylist.join(","));
        args.push(s)
      }
      _ => {}
    }

    match &self.allow_ffi {
      Some(ffi_allowlist) if ffi_allowlist.is_empty() => {
        args.push("--allow-ffi".to_string());
      }
      Some(ffi_allowlist) => {
        let s = format!("--allow-ffi={}", join_paths(ffi_allowlist, ","));
        args.push(s);
      }
      _ => {}
    }

    match &self.deny_ffi {
      Some(ffi_denylist) if ffi_denylist.is_empty() => {
        args.push("--deny-ffi".to_string());
      }
      Some(ffi_denylist) => {
        let s = format!("--deny-ffi={}", join_paths(ffi_denylist, ","));
        args.push(s);
      }
      _ => {}
    }

    if self.allow_hrtime {
      args.push("--allow-hrtime".to_string());
    }

    if self.deny_hrtime {
      args.push("--deny-hrtime".to_string());
    }

    args
  }

  /// Extract path arguments for config search paths.
  /// If it returns Some(vec), the config should be discovered
  /// from the passed `current_dir` after trying to discover from each entry in
  /// the returned vector.
  /// If it returns None, the config file shouldn't be discovered at all.
  pub fn config_path_args(
    &self,
    current_dir: &Path,
  ) -> Option<Vec<PathBuf>> {
    use DenoSubcommand::*;

    match &self.subcommand {
      Fmt(FmtFlags { files, .. }) => {
        Some(files.include.iter().map(|p| current_dir.join(p)).collect())
      }
      Lint(LintFlags { files, .. }) => {
        Some(files.include.iter().map(|p| current_dir.join(p)).collect())
      }
      Run(RunFlags { script, .. }) => {
        if let Ok(module_specifier) = resolve_url_or_path(script, current_dir) {
          if module_specifier.scheme() == "file" || module_specifier.scheme() == "npm" {
            if let Ok(p) = module_specifier.to_file_path() {
              Some(vec![p])
            } else {
              Some(vec![])
            }
          } else {
            // When the entrypoint doesn't have file: scheme (it's the remote
            // script), then we don't auto discover config file.
            None
          }
        } else {
          Some(vec![])
        }
      }
      Task(TaskFlags {
        cwd: Some(path), ..
      }) => {
        // todo(dsherret): Why is this canonicalized? Document why.
        // attempt to resolve the config file from the task subcommand's
        // `--cwd` when specified
        match canonicalize_path(&PathBuf::from(path)) {
          Ok(path) => Some(vec![path]),
          Err(_) => Some(vec![]),
        }
      }
      _ => Some(vec![]),
    }
  }

  /// Extract path argument for `package.json` search paths.
  /// If it returns Some(path), the `package.json` should be discovered
  /// from the `path` dir.
  /// If it returns None, the `package.json` file shouldn't be discovered at
  /// all.
  pub fn package_json_search_dir(
    &self,
    current_dir: &Path,
  ) -> Option<PathBuf> {
    use DenoSubcommand::*;

    match &self.subcommand {
      Run(RunFlags { script, .. }) => {
        let module_specifier = resolve_url_or_path(script, current_dir).ok()?;
        if module_specifier.scheme() == "file" {
          let p = module_specifier
            .to_file_path()
            .unwrap()
            .parent()?
            .to_owned();
          Some(p)
        } else if module_specifier.scheme() == "npm" {
          Some(current_dir.to_path_buf())
        } else {
          None
        }
      }
      Task(TaskFlags { cwd: Some(cwd), .. }) => resolve_url_or_path(cwd, current_dir)
        .ok()?
        .to_file_path()
        .ok(),
      Task(_) | Check(_) | Coverage(_) | Cache(_) | Info(_) | Eval(_) | Test(_) | Bench(_)
      | Repl(_) | Compile(_) | Publish(_) => std::env::current_dir().ok(),
      Add(_) | Bundle(_) | Completions(_) | Doc(_) | Fmt(_) | Init(_) | Install(_)
      | Uninstall(_) | Jupyter(_) | Lsp | Lint(_) | Types | Upgrade(_) | Vendor(_) => None,
    }
  }

  pub fn has_permission(&self) -> bool {
    self.allow_all
      || self.allow_hrtime
      || self.deny_hrtime
      || self.allow_env.is_some()
      || self.deny_env.is_some()
      || self.allow_ffi.is_some()
      || self.deny_ffi.is_some()
      || self.allow_net.is_some()
      || self.deny_net.is_some()
      || self.allow_read.is_some()
      || self.deny_read.is_some()
      || self.allow_run.is_some()
      || self.deny_run.is_some()
      || self.allow_sys.is_some()
      || self.deny_sys.is_some()
      || self.allow_write.is_some()
      || self.deny_write.is_some()
  }

  pub fn has_permission_in_argv(&self) -> bool {
    self.argv.iter().any(|arg| {
      arg == "--allow-all"
        || arg == "--allow-hrtime"
        || arg == "--deny-hrtime"
        || arg.starts_with("--allow-env")
        || arg.starts_with("--deny-env")
        || arg.starts_with("--allow-ffi")
        || arg.starts_with("--deny-ffi")
        || arg.starts_with("--allow-net")
        || arg.starts_with("--deny-net")
        || arg.starts_with("--allow-read")
        || arg.starts_with("--deny-read")
        || arg.starts_with("--allow-run")
        || arg.starts_with("--deny-run")
        || arg.starts_with("--allow-sys")
        || arg.starts_with("--deny-sys")
        || arg.starts_with("--allow-write")
        || arg.starts_with("--deny-write")
    })
  }
}

static ENV_VARIABLES_HELP: &str = color_print::cstr!(
  r#"<y>Environment variables:</>
    <g>DENO_AUTH_TOKENS</>     A semi-colon separated list of bearer tokens and
                         hostnames to use when fetching remote modules from
                         private repositories
                         (e.g. "abcde12345@deno.land;54321edcba@github.com")

    <g>DENO_TLS_CA_STORE</>    Comma-separated list of order dependent certificate
                         stores. Possible values: "system", "mozilla".
                         Defaults to "mozilla".

    <g>DENO_CERT</>            Load certificate authority from PEM encoded file

    <g>DENO_DIR</>             Set the cache directory

    <g>DENO_INSTALL_ROOT</>    Set deno install's output directory
                         (defaults to $HOME/.deno/bin)

    <g>DENO_REPL_HISTORY</>    Set REPL history file path
                         History file is disabled when the value is empty
                         (defaults to $DENO_DIR/deno_history.txt)

    <g>DENO_NO_PACKAGE_JSON</> Disables auto-resolution of package.json

    <g>DENO_NO_PROMPT</>       Set to disable permission prompts on access
                         (alternative to passing --no-prompt on invocation)

    <g>DENO_NO_UPDATE_CHECK</> Set to disable checking if a newer Deno version is
                         available

    <g>DENO_V8_FLAGS</>        Set V8 command line options

    <g>DENO_WEBGPU_TRACE</>    Directory to use for wgpu traces

    <g>DENO_JOBS</>            Number of parallel workers used for the --parallel
                         flag with the test subcommand. Defaults to number
                         of available CPUs.

    <g>HTTP_PROXY</>           Proxy address for HTTP requests
                         (module downloads, fetch)

    <g>HTTPS_PROXY</>          Proxy address for HTTPS requests
                         (module downloads, fetch)

    <g>NPM_CONFIG_REGISTRY</>  URL to use for the npm registry.

    <g>NO_COLOR</>             Set to disable color

    <g>NO_PROXY</>             Comma-separated list of hosts which do not use a proxy
                         (module downloads, fetch)"#
);

static DENO_HELP: &str = concat!(
  color_print::cstr!("<g>A modern JavaScript and TypeScript runtime</>"),
  "

Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  color_print::cstr!(
    "
Modules: https://deno.land/std/ https://deno.land/x/
Bugs: https://github.com/denoland/deno/issues

To start the REPL:

  <g>deno</>

To execute a script:

  <g>deno run https://examples.deno.land/hello-world.ts</>

To evaluate code in the shell:

  <g>deno eval \"console.log(30933 + 404)\"</>
"
  )
);

/// Main entry point for parsing deno's command line flags.
pub fn flags_from_vec(args: Vec<String>) -> clap::error::Result<Flags> {
  let mut app = clap_root();
  let mut matches = app.try_get_matches_from_mut(&args)?;

  let mut flags = Flags::default();

  if matches.get_flag("unstable") {
    flags.unstable_config.legacy_flag_enabled = true;
  }

  for (name, _, _) in crate::deno_cli::UNSTABLE_GRANULAR_FLAGS {
    if matches.get_flag(&format!("unstable-{}", name)) {
      flags.unstable_config.features.push(name.to_string());
    }
  }

  flags.unstable_config.bare_node_builtins = matches.get_flag("unstable-bare-node-builtins");
  flags.unstable_config.byonm = matches.get_flag("unstable-byonm");
  flags.unstable_config.sloppy_imports = matches.get_flag("unstable-sloppy-imports");

  if matches.get_flag("quiet") {
    flags.log_level = Some(Level::Error);
  } else if let Some(log_level) = matches.get_one::<String>("log-level") {
    flags.log_level = match log_level.as_str() {
      "trace" => Some(Level::Trace),
      "debug" => Some(Level::Debug),
      "info" => Some(Level::Info),
      _ => unreachable!(),
    };
  }

  if let Some((subcommand, mut m)) = matches.remove_subcommand() {
    match subcommand.as_str() {
      "add" => add_parse(&mut flags, &mut m),
      "bench" => bench_parse(&mut flags, &mut m),
      "bundle" => bundle_parse(&mut flags, &mut m),
      "cache" => cache_parse(&mut flags, &mut m),
      "check" => check_parse(&mut flags, &mut m),
      "compile" => compile_parse(&mut flags, &mut m),
      "completions" => completions_parse(&mut flags, &mut m, app),
      "coverage" => coverage_parse(&mut flags, &mut m),
      "doc" => doc_parse(&mut flags, &mut m),
      "eval" => eval_parse(&mut flags, &mut m),
      "fmt" => fmt_parse(&mut flags, &mut m),
      "init" => init_parse(&mut flags, &mut m),
      "info" => info_parse(&mut flags, &mut m),
      "install" => install_parse(&mut flags, &mut m),
      "jupyter" => jupyter_parse(&mut flags, &mut m),
      "lint" => lint_parse(&mut flags, &mut m),
      "lsp" => lsp_parse(&mut flags, &mut m),
      "repl" => repl_parse(&mut flags, &mut m),
      "run" => run_parse(&mut flags, &mut m, app)?,
      "task" => task_parse(&mut flags, &mut m),
      "test" => test_parse(&mut flags, &mut m),
      "types" => types_parse(&mut flags, &mut m),
      "uninstall" => uninstall_parse(&mut flags, &mut m),
      "upgrade" => upgrade_parse(&mut flags, &mut m),
      "vendor" => vendor_parse(&mut flags, &mut m),
      "publish" => publish_parse(&mut flags, &mut m),
      _ => unreachable!(),
    }
  } else {
    handle_repl_flags(
      &mut flags,
      ReplFlags {
        eval_files: None,
        eval: None,
        is_default_command: true,
      },
    )
  }

  Ok(flags)
}

fn handle_repl_flags(
  flags: &mut Flags,
  repl_flags: ReplFlags,
) {
  // If user runs just `deno` binary we enter REPL and allow all permissions.
  if repl_flags.is_default_command {
    flags.allow_net = Some(vec![]);
    flags.allow_env = Some(vec![]);
    flags.allow_run = Some(vec![]);
    flags.allow_read = Some(vec![]);
    flags.allow_sys = Some(vec![]);
    flags.allow_write = Some(vec![]);
    flags.allow_ffi = Some(vec![]);
    flags.allow_hrtime = true;
  }
  flags.subcommand = DenoSubcommand::Repl(repl_flags);
}

fn clap_root() -> Command {
  let long_version = format!(
    "{} ({}, {})\nv8 {}\ntypescript {}",
    crate::deno_cli::version::deno(),
    if crate::deno_cli::version::is_canary() {
      "canary"
    } else {
      "PROFILE"
    },
    "release",
    deno_core::v8_version(),
    crate::deno_cli::version::TYPESCRIPT
  );

  let mut cmd = Command::new("deno")
    .bin_name("deno")
    .styles(
      clap::builder::Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::White.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
    )
    .color(ColorChoice::Auto)
    .max_term_width(80)
    .version(crate::deno_cli::version::deno())
    .long_version(long_version)
    // cause --unstable flags to display at the bottom of the help text
    .next_display_order(1000)
    .arg(
      Arg::new("unstable")
        .long("unstable")
        .help("Enable unstable features and APIs")
        .action(ArgAction::SetTrue)
        .global(true),
    )
    .arg(
      Arg::new("unstable-bare-node-builtins")
        .long("unstable-bare-node-builtins")
        .help("Enable unstable bare node builtins feature")
        .env("DENO_UNSTABLE_BARE_NODE_BUILTINS")
        .value_parser(FalseyValueParser::new())
        .action(ArgAction::SetTrue)
        .global(true),
    )
    .arg(
      Arg::new("unstable-byonm")
        .long("unstable-byonm")
        .help("Enable unstable 'bring your own node_modules' feature")
        .env("DENO_UNSTABLE_BYONM")
        .value_parser(FalseyValueParser::new())
        .action(ArgAction::SetTrue)
        .global(true),
    )
    .arg(
      Arg::new("unstable-sloppy-imports")
        .long("unstable-sloppy-imports")
        .help(
          "Enable unstable resolving of specifiers by extension probing, .js to .ts, and directory probing.",
        )
        .env("DENO_UNSTABLE_SLOPPY_IMPORTS")
        .value_parser(FalseyValueParser::new())
        .action(ArgAction::SetTrue)
        .global(true),
    );

  for (flag_name, help, _) in crate::deno_cli::UNSTABLE_GRANULAR_FLAGS {
    cmd = cmd.arg(
      Arg::new(format!("unstable-{}", flag_name))
        .long(format!("unstable-{}", flag_name))
        .help(help)
        .action(ArgAction::SetTrue)
        .global(true),
    );
  }

  cmd
    // reset the display order after the unstable flags
    .next_display_order(0)
    .arg(
      Arg::new("log-level")
        .short('L')
        .long("log-level")
        .help("Set log level")
        .hide(true)
        .value_parser(["trace", "debug", "info"])
        .global(true),
    )
    .arg(
      Arg::new("quiet")
        .short('q')
        .long("quiet")
        .help("Suppress diagnostic output")
        .action(ArgAction::SetTrue)
        .global(true),
    )
    .subcommand(run_subcommand())
    .defer(|cmd| {
      cmd
        .subcommand(add_subcommand())
        .subcommand(bench_subcommand())
        .subcommand(bundle_subcommand())
        .subcommand(cache_subcommand())
        .subcommand(check_subcommand())
        .subcommand(compile_subcommand())
        .subcommand(completions_subcommand())
        .subcommand(coverage_subcommand())
        .subcommand(doc_subcommand())
        .subcommand(eval_subcommand())
        .subcommand(fmt_subcommand())
        .subcommand(init_subcommand())
        .subcommand(info_subcommand())
        .subcommand(install_subcommand())
        .subcommand(jupyter_subcommand())
        .subcommand(uninstall_subcommand())
        .subcommand(lsp_subcommand())
        .subcommand(lint_subcommand())
        .subcommand(publish_subcommand())
        .subcommand(repl_subcommand())
        .subcommand(task_subcommand())
        .subcommand(test_subcommand())
        .subcommand(types_subcommand())
        .subcommand(upgrade_subcommand())
        .subcommand(vendor_subcommand())
    })
    .long_about(DENO_HELP)
    .after_help(ENV_VARIABLES_HELP)
}

fn add_subcommand() -> Command {
  Command::new("add")
    .about("Add dependencies")
    .long_about(
      "Add dependencies to the configuration file.

  deno add @std/path

You can add multiple dependencies at once:

  deno add @std/path @std/assert
",
    )
    .defer(|cmd| {
      cmd.arg(
        Arg::new("packages")
          .help("List of packages to add")
          .required(true)
          .num_args(1..)
          .action(ArgAction::Append),
      )
    })
}

fn bench_subcommand() -> Command {
  Command::new("bench")
    .about("Run benchmarks")
    .long_about(
      "Run benchmarks using Deno's built-in bench tool.

Evaluate the given modules, run all benches declared with 'Deno.bench()'
and report results to standard output:

  deno bench src/fetch_bench.ts src/signal_bench.ts

Directory arguments are expanded to all contained files matching the
glob {*_,*.,}bench.{js,mjs,ts,mts,jsx,tsx}:

  deno bench src/",
    )
    .defer(|cmd| {
      runtime_args(cmd, true, false)
        .arg(check_arg(true))
        .arg(
          Arg::new("json")
            .long("json")
            .action(ArgAction::SetTrue)
            .help("UNSTABLE: Output benchmark result in JSON format"),
        )
        .arg(
          Arg::new("ignore")
            .long("ignore")
            .num_args(1..)
            .use_value_delimiter(true)
            .require_equals(true)
            .help("Ignore files"),
        )
        .arg(
          Arg::new("filter")
            .long("filter")
            .allow_hyphen_values(true)
            .help("Run benchmarks with this string or pattern in the bench name"),
        )
        .arg(
          Arg::new("files")
            .help("List of file names to run")
            .num_args(..)
            .action(ArgAction::Append),
        )
        .arg(
          Arg::new("no-run")
            .long("no-run")
            .help("Cache bench modules, but don't run benchmarks")
            .action(ArgAction::SetTrue),
        )
        .arg(watch_arg(false))
        .arg(no_clear_screen_arg())
        .arg(script_arg().last(true))
        .arg(env_file_arg())
    })
}

fn bundle_subcommand() -> Command {
  Command::new("bundle")
    .about("Bundle module and dependencies into single file")
    .long_about(
      "⚠️ Warning: `deno bundle` is deprecated and will be removed in Deno 2.0.
Use an alternative bundler like \"deno_emit\", \"esbuild\" or \"rollup\" instead.

Output a single JavaScript file with all dependencies.

  deno bundle https://deno.land/std/http/file_server.ts file_server.bundle.js

If no output file is given, the output is written to standard output:

  deno bundle https://deno.land/std/http/file_server.ts",
    )
    .defer(|cmd| {
      compile_args(cmd)
        .hide(true)
        .arg(check_arg(true))
        .arg(
          Arg::new("source_file")
            .required(true)
            .value_hint(ValueHint::FilePath),
        )
        .arg(Arg::new("out_file").value_hint(ValueHint::FilePath))
        .arg(watch_arg(false))
        .arg(no_clear_screen_arg())
        .arg(executable_ext_arg())
    })
}

fn cache_subcommand() -> Command {
  Command::new("cache")
    .about("Cache the dependencies")
    .long_about(
      "Cache and compile remote dependencies recursively.

Download and compile a module with all of its static dependencies and save
them in the local cache, without running any code:

  deno cache https://deno.land/std/http/file_server.ts

Future runs of this module will trigger no downloads or compilation unless
--reload is specified.",
    )
    .defer(|cmd| {
      compile_args(cmd).arg(check_arg(false)).arg(
        Arg::new("file")
          .num_args(1..)
          .required(true)
          .value_hint(ValueHint::FilePath),
      )
    })
}

fn check_subcommand() -> Command {
  Command::new("check")
    .about("Type-check the dependencies")
    .long_about(
      "Download and type-check without execution.

  deno check https://deno.land/std/http/file_server.ts

Unless --reload is specified, this command will not re-download already cached dependencies.",
    )
    .defer(|cmd| {
      compile_args_without_check_args(cmd)
        .arg(
          Arg::new("all")
            .long("all")
            .help("Type-check all code, including remote modules and npm packages")
            .action(ArgAction::SetTrue)
            .conflicts_with("no-remote"),
        )
        .arg(
          // past alias for --all
          Arg::new("remote")
            .long("remote")
            .help("Type-check all modules, including remote")
            .action(ArgAction::SetTrue)
            .conflicts_with("no-remote")
            .hide(true),
        )
        .arg(
          Arg::new("file")
            .num_args(1..)
            .required(true)
            .value_hint(ValueHint::FilePath),
        )
    })
}

fn compile_subcommand() -> Command {
  Command::new("compile")
    .about("Compile the script into a self contained executable")
    .long_about(
      "Compiles the given script into a self contained executable.

  deno compile -A https://deno.land/std/http/file_server.ts
  deno compile --output file_server https://deno.land/std/http/file_server.ts

Any flags passed which affect runtime behavior, such as '--unstable',
'--allow-*', '--v8-flags', etc. are encoded into the output executable and
used at runtime as if they were passed to a similar 'deno run' command.

The executable name is inferred by default: Attempt to take the file stem of
the URL path. The above example would become 'file_server'. If the file stem
is something generic like 'main', 'mod', 'index' or 'cli', and the path has no
parent, take the file name of the parent path. Otherwise settle with the
generic name. If the resulting name has an '@...' suffix, strip it.

Cross-compiling to different target architectures is supported using the
`--target` flag. On the first invocation with deno will download proper
binary and cache it in $DENO_DIR. The aarch64-apple-darwin target is not
supported in canary.
",
    )
    .defer(|cmd| {
      runtime_args(cmd, true, false)
        .arg(check_arg(true))
        .arg(
          Arg::new("include")
            .long("include")
            .help("Additional module to include in the module graph")
            .long_help(
              "Includes an additional module in the compiled executable's module
    graph. Use this flag if a dynamically imported module or a web worker main
    module fails to load in the executable. This flag can be passed multiple
    times, to include multiple additional modules.",
            )
            .action(ArgAction::Append)
            .value_hint(ValueHint::FilePath),
        )
        .arg(
          Arg::new("output")
            .long("output")
            .short('o')
            // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
            .value_parser(value_parser!(PathBuf))
            .help("Output file (defaults to $PWD/<inferred-name>)")
            .value_hint(ValueHint::FilePath),
        )
        .arg(
          Arg::new("target")
            .long("target")
            .help("Target OS architecture")
            .value_parser([
              "x86_64-unknown-linux-gnu",
              "aarch64-unknown-linux-gnu",
              "x86_64-pc-windows-msvc",
              "x86_64-apple-darwin",
              "aarch64-apple-darwin",
            ]),
        )
        .arg(
          Arg::new("no-terminal")
            .long("no-terminal")
            .help("Hide terminal on Windows")
            .action(ArgAction::SetTrue),
        )
        .arg(executable_ext_arg())
        .arg(env_file_arg())
        .arg(script_arg().required(true).trailing_var_arg(true))
    })
}

fn completions_subcommand() -> Command {
  Command::new("completions")
    .about("Generate shell completions")
    .long_about(
      "Output shell completion script to standard output.

  deno completions bash > /usr/local/etc/bash_completion.d/deno.bash
  source /usr/local/etc/bash_completion.d/deno.bash",
    )
    .defer(|cmd| {
      cmd.disable_help_subcommand(true).arg(
        Arg::new("shell")
          .value_parser(["bash", "fish", "powershell", "zsh", "fig"])
          .required(true),
      )
    })
}

fn coverage_subcommand() -> Command {
  Command::new("coverage")
    .about("Print coverage reports")
    .long_about(
      "Print coverage reports from coverage profiles.

Collect a coverage profile with deno test:

  deno test --coverage=cov_profile

Print a report to stdout:

  deno coverage cov_profile

Include urls that start with the file schema:

  deno coverage --include=\"^file:\" cov_profile

Exclude urls ending with test.ts and test.js:

  deno coverage --exclude=\"test\\.(ts|js)\" cov_profile

Include urls that start with the file schema and exclude files ending with
test.ts and test.js, for an url to match it must match the include pattern and
not match the exclude pattern:

  deno coverage --include=\"^file:\" --exclude=\"test\\.(ts|js)\" cov_profile

Write a report using the lcov format:

  deno coverage --lcov --output=cov.lcov cov_profile/

Generate html reports from lcov:

  genhtml -o html_cov cov.lcov
",
    )
    .defer(|cmd| {
      cmd
        .arg(
          Arg::new("ignore")
            .long("ignore")
            .num_args(1..)
            .use_value_delimiter(true)
            .require_equals(true)
            .help("Ignore coverage files")
            .value_hint(ValueHint::AnyPath),
        )
        .arg(
          Arg::new("include")
            .long("include")
            .num_args(1..)
            .action(ArgAction::Append)
            .value_name("regex")
            .require_equals(true)
            .default_value(r"^file:")
            .help("Include source files in the report"),
        )
        .arg(
          Arg::new("exclude")
            .long("exclude")
            .num_args(1..)
            .action(ArgAction::Append)
            .value_name("regex")
            .require_equals(true)
            .default_value(r"test\.(js|mjs|ts|jsx|tsx)$")
            .help("Exclude source files from the report"),
        )
        .arg(
          Arg::new("lcov")
            .long("lcov")
            .help("Output coverage report in lcov format")
            .action(ArgAction::SetTrue),
        )
        .arg(
          Arg::new("output")
            .requires("lcov")
            .long("output")
            // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
            .value_parser(value_parser!(PathBuf))
            .help("Output file (defaults to stdout) for lcov")
            .long_help(
              "Exports the coverage report in lcov format to the given file.
    Filename should be passed along with '=' For example '--output=foo.lcov'
    If no --output arg is specified then the report is written to stdout.",
            )
            .require_equals(true)
            .value_hint(ValueHint::FilePath),
        )
        .arg(
          Arg::new("html")
            .long("html")
            .help("Output coverage report in HTML format in the given directory")
            .action(ArgAction::SetTrue),
        )
        .arg(
          Arg::new("detailed")
            .long("detailed")
            .help("Output coverage report in detailed format in the terminal.")
            .action(ArgAction::SetTrue),
        )
        .arg(
          Arg::new("files")
            .num_args(0..)
            .action(ArgAction::Append)
            .value_hint(ValueHint::AnyPath),
        )
    })
}

fn doc_subcommand() -> Command {
  Command::new("doc")
    .about("Show documentation for a module")
    .long_about(
      "Show documentation for a module.

Output documentation to standard output:

    deno doc ./path/to/module.ts

Output documentation in HTML format:

    deno doc --html --name=\"My library\" ./path/to/module.ts
    deno doc --html --name=\"My library\" ./main.ts ./dev.ts
    deno doc --html --name=\"My library\" --output=./documentation/ ./path/to/module.ts

Output private documentation to standard output:

    deno doc --private ./path/to/module.ts

Output documentation in JSON format:

    deno doc --json ./path/to/module.ts

Lint a module for documentation diagnostics:

    deno doc --lint ./path/to/module.ts

Target a specific symbol:

    deno doc ./path/to/module.ts MyClass.someField

Show documentation for runtime built-ins:

    deno doc
    deno doc --builtin Deno.Listener",
    )
    .defer(|cmd| {
      cmd
        .arg(import_map_arg())
        .arg(reload_arg())
        .arg(lock_arg())
        .arg(no_lock_arg())
        .arg(no_npm_arg())
        .arg(no_remote_arg())
        .arg(
          Arg::new("json")
            .long("json")
            .help("Output documentation in JSON format")
            .action(ArgAction::SetTrue),
        )
        .arg(
          Arg::new("html")
            .long("html")
            .help("Output documentation in HTML format")
            .action(ArgAction::SetTrue)
            .conflicts_with("json"),
        )
        .arg(
          Arg::new("name")
            .long("name")
            .help("The name that will be displayed in the docs")
            .action(ArgAction::Set)
            .required_if_eq("html", "true")
            .require_equals(true),
        )
        .arg(
          Arg::new("output")
            .long("output")
            .help("Directory for HTML documentation output")
            .action(ArgAction::Set)
            .require_equals(true)
            .value_hint(ValueHint::DirPath)
            // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("private")
            .long("private")
            .help("Output private documentation")
            .action(ArgAction::SetTrue),
        )
        .arg(
          Arg::new("filter")
            .long("filter")
            .help("Dot separated path to symbol")
            .required(false)
            .conflicts_with("json")
            .conflicts_with("lint")
            .conflicts_with("html"),
        )
        .arg(
          Arg::new("lint")
            .long("lint")
            .help("Output documentation diagnostics.")
            .action(ArgAction::SetTrue),
        )
        // TODO(nayeemrmn): Make `--builtin` a proper option. Blocked by
        // https://github.com/clap-rs/clap/issues/1794. Currently `--builtin` is
        // just a possible value of `source_file` so leading hyphens must be
        // enabled.
        .allow_hyphen_values(true)
        .arg(
          Arg::new("source_file")
            .num_args(1..)
            .action(ArgAction::Append)
            .value_hint(ValueHint::FilePath)
            .required_if_eq_any([("html", "true"), ("lint", "true")]),
        )
    })
}

fn eval_subcommand() -> Command {
  Command::new("eval")
    .about("Eval script")
    .long_about(
      "Evaluate JavaScript from the command line.

  deno eval \"console.log('hello world')\"

To evaluate as TypeScript:

  deno eval --ext=ts \"const v: string = 'hello'; console.log(v)\"

This command has implicit access to all permissions (--allow-all).",
    )
    .defer(|cmd| {
      runtime_args(cmd, false, true)
        .arg(check_arg(false))
        .arg(
          // TODO(@satyarohith): remove this argument in 2.0.
          Arg::new("ts")
            .conflicts_with("ext")
            .long("ts")
            .short('T')
            .help("deprecated: Use `--ext=ts` instead. The `--ts` and `-T` flags are deprecated and will be removed in Deno 2.0.")
            .action(ArgAction::SetTrue)
            .hide(true),
        )
        .arg(executable_ext_arg())
        .arg(
          Arg::new("print")
            .long("print")
            .short('p')
            .help("print result to stdout")
            .action(ArgAction::SetTrue),
        )
        .arg(
          Arg::new("code_arg")
            .num_args(1..)
            .action(ArgAction::Append)
            .help("Code arg")
            .value_name("CODE_ARG")
            .required(true),
        )
        .arg(env_file_arg())
    })
}

fn fmt_subcommand() -> Command {
  Command::new("fmt")
    .about("Format source files")
    .long_about(
      "Auto-format JavaScript, TypeScript, Markdown, and JSON files.

  deno fmt
  deno fmt myfile1.ts myfile2.ts
  deno fmt --check

Format stdin and write to stdout:

  cat file.ts | deno fmt -

Ignore formatting code by preceding it with an ignore comment:

  // deno-fmt-ignore

Ignore formatting a file by adding an ignore comment at the top of the file:

  // deno-fmt-ignore-file",
    )
    .defer(|cmd| {
      cmd
        .arg(config_arg())
        .arg(no_config_arg())
        .arg(
          Arg::new("check")
            .long("check")
            .help("Check if the source files are formatted")
            .num_args(0),
        )
        .arg(
          Arg::new("ext")
            .long("ext")
            .help("Set content type of the supplied file")
            // prefer using ts for formatting instead of js because ts works in more scenarios
            .default_value("ts")
            .value_parser(["ts", "tsx", "js", "jsx", "md", "json", "jsonc", "ipynb"]),
        )
        .arg(
          Arg::new("ignore")
            .long("ignore")
            .num_args(1..)
            .use_value_delimiter(true)
            .require_equals(true)
            .help("Ignore formatting particular source files")
            .value_hint(ValueHint::AnyPath),
        )
        .arg(
          Arg::new("files")
            .num_args(1..)
            .action(ArgAction::Append)
            .required(false)
            .value_hint(ValueHint::AnyPath),
        )
        .arg(watch_arg(false))
        .arg(no_clear_screen_arg())
        .arg(
          Arg::new("use-tabs")
            .long("use-tabs")
            .alias("options-use-tabs")
            .num_args(0..=1)
            .value_parser(value_parser!(bool))
            .default_missing_value("true")
            .require_equals(true)
            .help("Use tabs instead of spaces for indentation. Defaults to false."),
        )
        .arg(
          Arg::new("line-width")
            .long("line-width")
            .alias("options-line-width")
            .help("Define maximum line width. Defaults to 80.")
            .value_parser(value_parser!(NonZeroU32)),
        )
        .arg(
          Arg::new("indent-width")
            .long("indent-width")
            .alias("options-indent-width")
            .help("Define indentation width. Defaults to 2.")
            .value_parser(value_parser!(NonZeroU8)),
        )
        .arg(
          Arg::new("single-quote")
            .long("single-quote")
            .alias("options-single-quote")
            .num_args(0..=1)
            .value_parser(value_parser!(bool))
            .default_missing_value("true")
            .require_equals(true)
            .help("Use single quotes. Defaults to false."),
        )
        .arg(
          Arg::new("prose-wrap")
            .long("prose-wrap")
            .alias("options-prose-wrap")
            .value_parser(["always", "never", "preserve"])
            .help("Define how prose should be wrapped. Defaults to always."),
        )
        .arg(
          Arg::new("no-semicolons")
            .long("no-semicolons")
            .alias("options-no-semicolons")
            .num_args(0..=1)
            .value_parser(value_parser!(bool))
            .default_missing_value("true")
            .require_equals(true)
            .help("Don't use semicolons except where necessary. Defaults to false."),
        )
    })
}

fn init_subcommand() -> Command {
  Command::new("init")
    .about("Initialize a new project")
    .defer(|cmd| {
      cmd.arg(
        Arg::new("dir")
          .required(false)
          .value_hint(ValueHint::DirPath),
      )
    })
}

fn info_subcommand() -> Command {
  Command::new("info")
      .about("Show info about cache or info related to source file")
      .long_about(
        "Information about a module or the cache directories.

Get information about a module:

  deno info https://deno.land/std/http/file_server.ts

The following information is shown:

local: Local path of the file.
type: JavaScript, TypeScript, or JSON.
emit: Local path of compiled source code. (TypeScript only.)
dependencies: Dependency tree of the source file.

Without any additional arguments, 'deno info' shows:

DENO_DIR: Directory containing Deno-managed files.
Remote modules cache: Subdirectory containing downloaded remote modules.
TypeScript compiler cache: Subdirectory containing TS compiler output.",
      )
    .defer(|cmd| cmd
      .arg(Arg::new("file").required(false).value_hint(ValueHint::FilePath))
      .arg(reload_arg().requires("file"))
      .arg(ca_file_arg())
      .arg(
        location_arg()
          .conflicts_with("file")
          .help("Show files used for origin bound APIs like the Web Storage API when running a script with '--location=<HREF>'")
      )
      .arg(no_check_arg().hide(true)) // TODO(lucacasonato): remove for 2.0
      .arg(no_config_arg())
      .arg(no_remote_arg())
      .arg(no_npm_arg())
      .arg(lock_arg())
      .arg(lock_write_arg())
      .arg(no_lock_arg())
      .arg(config_arg())
      .arg(import_map_arg())
      .arg(node_modules_dir_arg())
      .arg(vendor_arg())
      .arg(
        Arg::new("json")
          .long("json")
          .help("UNSTABLE: Outputs the information in JSON format")
          .action(ArgAction::SetTrue),
      ))
}

fn install_subcommand() -> Command {
  Command::new("install")
    .about("Install script as an executable")
    .long_about(
      "Installs a script as an executable in the installation root's bin directory.

  deno install --allow-net --allow-read https://deno.land/std/http/file_server.ts
  deno install https://examples.deno.land/color-logging.ts

To change the executable name, use -n/--name:

  deno install --allow-net --allow-read -n serve https://deno.land/std/http/file_server.ts

The executable name is inferred by default:
  - Attempt to take the file stem of the URL path. The above example would
    become 'file_server'.
  - If the file stem is something generic like 'main', 'mod', 'index' or 'cli',
    and the path has no parent, take the file name of the parent path. Otherwise
    settle with the generic name.
  - If the resulting name has an '@...' suffix, strip it.

To change the installation root, use --root:

  deno install --allow-net --allow-read --root /usr/local https://deno.land/std/http/file_server.ts

The installation root is determined, in order of precedence:
  - --root option
  - DENO_INSTALL_ROOT environment variable
  - $HOME/.deno

These must be added to the path manually if required.",
    )
    .defer(|cmd| {
      runtime_args(cmd, true, true)
        .arg(
          Arg::new("cmd")
            .required(true)
            .num_args(1..)
            .value_hint(ValueHint::FilePath),
        )
        .arg(check_arg(true))
        .arg(
          Arg::new("name")
            .long("name")
            .short('n')
            .help("Executable file name")
            .required(false),
        )
        .arg(
          Arg::new("root")
            .long("root")
            .help("Installation root")
            .value_hint(ValueHint::DirPath),
        )
        .arg(
          Arg::new("force")
            .long("force")
            .short('f')
            .help("Forcefully overwrite existing installation")
            .action(ArgAction::SetTrue),
        )
    })
    .arg(env_file_arg())
}

fn jupyter_subcommand() -> Command {
  Command::new("jupyter")
    .arg(
      Arg::new("install")
        .long("install")
        .help("Installs kernelspec, requires 'jupyter' command to be available.")
        .conflicts_with("kernel")
        .action(ArgAction::SetTrue),
    )
    .arg(
      Arg::new("kernel")
        .long("kernel")
        .help("Start the kernel")
        .conflicts_with("install")
        .requires("conn")
        .action(ArgAction::SetTrue),
    )
    .arg(
      Arg::new("conn")
        .long("conn")
        .help("Path to JSON file describing connection parameters, provided by Jupyter")
        // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
        .value_parser(value_parser!(PathBuf))
        .value_hint(ValueHint::FilePath)
        .conflicts_with("install"),
    )
    .about("Deno kernel for Jupyter notebooks")
}

fn uninstall_subcommand() -> Command {
  Command::new("uninstall")
    .about("Uninstall a script previously installed with deno install")
    .long_about(
      "Uninstalls an executable script in the installation root's bin directory.

  deno uninstall serve

To change the installation root, use --root:

  deno uninstall --root /usr/local serve

The installation root is determined, in order of precedence:
  - --root option
  - DENO_INSTALL_ROOT environment variable
  - $HOME/.deno",
    )
    .defer(|cmd| {
      cmd.arg(Arg::new("name").required(true)).arg(
        Arg::new("root")
          .long("root")
          .help("Installation root")
          .value_hint(ValueHint::DirPath),
      )
    })
}

static LSP_HELP: &str = concat!(
  "The 'deno lsp' subcommand provides a way for code editors and IDEs to
interact with Deno using the Language Server Protocol. Usually humans do not
use this subcommand directly. For example, 'deno lsp' can provide IDEs with
go-to-definition support and automatic code formatting.

How to connect various editors and IDEs to 'deno lsp':
https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/getting_started/setup_your_environment#editors-and-ides",
);

fn lsp_subcommand() -> Command {
  Command::new("lsp")
    .about("Start the language server")
    .long_about(LSP_HELP)
}

fn lint_subcommand() -> Command {
  Command::new("lint")
    .about("Lint source files")
    .long_about(
      "Lint JavaScript/TypeScript source code.

  deno lint
  deno lint myfile1.ts myfile2.js

Print result as JSON:

  deno lint --json

Read from stdin:

  cat file.ts | deno lint -
  cat file.ts | deno lint --json -

List available rules:

  deno lint --rules

Ignore diagnostics on the next line by preceding it with an ignore comment and
rule name:

  // deno-lint-ignore no-explicit-any
  // deno-lint-ignore require-await no-empty

Names of rules to ignore must be specified after ignore comment.

Ignore linting a file by adding an ignore comment at the top of the file:

  // deno-lint-ignore-file
",
    )
    .defer(|cmd| {
      cmd
        .arg(
          Arg::new("rules")
            .long("rules")
            .help("List available rules")
            .action(ArgAction::SetTrue),
        )
        .arg(
          Arg::new("rules-tags")
            .long("rules-tags")
            .require_equals(true)
            .num_args(1..)
            .action(ArgAction::Append)
            .use_value_delimiter(true)
            .help("Use set of rules with a tag"),
        )
        .arg(
          Arg::new("rules-include")
            .long("rules-include")
            .require_equals(true)
            .num_args(1..)
            .use_value_delimiter(true)
            .conflicts_with("rules")
            .help("Include lint rules"),
        )
        .arg(
          Arg::new("rules-exclude")
            .long("rules-exclude")
            .require_equals(true)
            .num_args(1..)
            .use_value_delimiter(true)
            .conflicts_with("rules")
            .help("Exclude lint rules"),
        )
        .arg(no_config_arg())
        .arg(config_arg())
        .arg(
          Arg::new("ignore")
            .long("ignore")
            .num_args(1..)
            .use_value_delimiter(true)
            .require_equals(true)
            .help("Ignore linting particular source files")
            .value_hint(ValueHint::AnyPath),
        )
        .arg(
          Arg::new("json")
            .long("json")
            .help("Output lint result in JSON format")
            .action(ArgAction::SetTrue),
        )
        .arg(
          Arg::new("compact")
            .long("compact")
            .help("Output lint result in compact format")
            .action(ArgAction::SetTrue)
            .conflicts_with("json"),
        )
        .arg(
          Arg::new("files")
            .num_args(1..)
            .action(ArgAction::Append)
            .required(false)
            .value_hint(ValueHint::AnyPath),
        )
        .arg(watch_arg(false))
        .arg(no_clear_screen_arg())
    })
}

fn repl_subcommand() -> Command {
  Command::new("repl")
    .about("Read Eval Print Loop")
    .defer(|cmd| runtime_args(cmd, true, true)
      .arg(check_arg(false))
      .arg(
        Arg::new("eval-file")
          .long("eval-file")
          .num_args(1..)
          .use_value_delimiter(true)
          .require_equals(true)
          .help("Evaluates the provided file(s) as scripts when the REPL starts. Accepts file paths and URLs.")
          .value_hint(ValueHint::AnyPath),
      )
      .arg(
        Arg::new("eval")
          .long("eval")
          .help("Evaluates the provided code when the REPL starts.")
          .value_name("code"),
      ))
      .arg(env_file_arg())
}

fn run_subcommand() -> Command {
  runtime_args(Command::new("run"), true, true)
    .arg(check_arg(false))
    .arg(watch_arg(true))
    .arg(hmr_arg(true))
    .arg(no_clear_screen_arg())
    .arg(executable_ext_arg())
    .arg(
      script_arg()
        .required_unless_present("v8-flags")
        .trailing_var_arg(true),
    )
    .arg(env_file_arg())
    .about("Run a JavaScript or TypeScript program")
    .long_about(
      "Run a JavaScript or TypeScript program

By default all programs are run in sandbox without access to disk, network or
ability to spawn subprocesses.

  deno run https://examples.deno.land/hello-world.ts

Grant all permissions:

  deno run -A https://deno.land/std/http/file_server.ts

Grant permission to read from disk and listen to network:

  deno run --allow-read --allow-net https://deno.land/std/http/file_server.ts

Grant permission to read allow-listed files from disk:

  deno run --allow-read=/etc https://deno.land/std/http/file_server.ts

Specifying the filename '-' to read the file from stdin.

  curl https://examples.deno.land/hello-world.ts | deno run -",
    )
}

fn task_subcommand() -> Command {
  Command::new("task")
    .about("Run a task defined in the configuration file")
    .long_about(
      "Run a task defined in the configuration file

  deno task build",
    )
    .defer(|cmd| {
      cmd
        .allow_external_subcommands(true)
        .subcommand_value_name("TASK")
        .arg(config_arg())
        .arg(
          Arg::new("cwd")
            .long("cwd")
            .value_name("DIR")
            .help("Specify the directory to run the task in")
            .value_hint(ValueHint::DirPath),
        )
    })
}

fn test_subcommand() -> Command {
  Command::new("test")
    .about("Run tests")
    .long_about(
      "Run tests using Deno's built-in test runner.

Evaluate the given modules, run all tests declared with 'Deno.test()' and
report results to standard output:

  deno test src/fetch_test.ts src/signal_test.ts

Directory arguments are expanded to all contained files matching the glob
{*_,*.,}test.{js,mjs,ts,mts,jsx,tsx}:

  deno test src/",
    )
  .defer(|cmd| runtime_args(cmd, true, true)
    .arg(check_arg(true))
    .arg(
      Arg::new("ignore")
        .long("ignore")
        .num_args(1..)
        .use_value_delimiter(true)
        .require_equals(true)
        .help("Ignore files")
        .value_hint(ValueHint::AnyPath),
    )
    .arg(
      Arg::new("no-run")
        .long("no-run")
        .help("Cache test modules, but don't run tests")
        .action(ArgAction::SetTrue),
    )
    .arg(
      Arg::new("trace-ops")
        .long("trace-ops")
        .help("Deprecated alias for --trace-leaks.")
        .hide(true)
        .action(ArgAction::SetTrue),
    )
    .arg(
      Arg::new("trace-leaks")
        .long("trace-leaks")
        .help("Enable tracing of leaks. Useful when debugging leaking ops in test, but impacts test execution time.")
        .action(ArgAction::SetTrue),
    )
    .arg(
      Arg::new("doc")
        .long("doc")
        .help("Type-check code blocks in JSDoc and Markdown")
        .action(ArgAction::SetTrue),
    )
    .arg(
      Arg::new("fail-fast")
        .long("fail-fast")
        .alias("failfast")
        .help("Stop after N errors. Defaults to stopping after first failure.")
        .num_args(0..=1)
        .require_equals(true)
        .value_name("N")
        .value_parser(value_parser!(NonZeroUsize)),
    )
    .arg(
      Arg::new("allow-none")
        .long("allow-none")
        .help("Don't return error code if no test files are found")
        .action(ArgAction::SetTrue),
    )
    .arg(
      Arg::new("filter")
        .allow_hyphen_values(true)
        .long("filter")
        .help("Run tests with this string or pattern in the test name"),
    )
    .arg(
      Arg::new("shuffle")
        .long("shuffle")
        .value_name("NUMBER")
        .help("Shuffle the order in which the tests are run")
        .num_args(0..=1)
        .require_equals(true)
        .value_parser(value_parser!(u64)),
    )
    .arg(
      Arg::new("coverage")
        .long("coverage")
        .value_name("DIR")
        .num_args(0..=1)
        .require_equals(true)
        .default_missing_value("coverage")
        .conflicts_with("inspect")
        .conflicts_with("inspect-wait")
        .conflicts_with("inspect-brk")
        .help("Collect coverage profile data into DIR. If DIR is not specified, it uses 'coverage/'."),
    )
    .arg(
      Arg::new("parallel")
        .long("parallel")
        .help("Run test modules in parallel. Parallelism defaults to the number of available CPUs or the value in the DENO_JOBS environment variable.")
        .conflicts_with("jobs")
        .action(ArgAction::SetTrue)
    )
    .arg(
      Arg::new("jobs")
        .short('j')
        .long("jobs")
        .help("deprecated: The `--jobs` flag is deprecated and will be removed in Deno 2.0. Use the `--parallel` flag with possibly the `DENO_JOBS` environment variable instead.")
        .hide(true)
        .num_args(0..=1)
        .value_parser(value_parser!(NonZeroUsize)),
    )
    .arg(
      Arg::new("files")
        .help("List of file names to run")
        .num_args(0..)
        .action(ArgAction::Append)
        .value_hint(ValueHint::AnyPath),
    )
    .arg(
      watch_arg(false)
        .conflicts_with("no-run")
        .conflicts_with("coverage"),
    )
    .arg(no_clear_screen_arg())
    .arg(script_arg().last(true))
    .arg(
      Arg::new("junit-path")
        .long("junit-path")
        .value_name("PATH")
        .value_hint(ValueHint::FilePath)
        .help("Write a JUnit XML test report to PATH. Use '-' to write to stdout which is the default when PATH is not provided.")
    )
    .arg(
      Arg::new("reporter")
        .long("reporter")
        .help("Select reporter to use. Default to 'pretty'.")
        .value_parser(["pretty", "dot", "junit", "tap"])
    )
    .arg(env_file_arg())
  )
}

fn types_subcommand() -> Command {
  Command::new("types")
    .about("Print runtime TypeScript declarations")
    .long_about(
      "Print runtime TypeScript declarations.

  deno types > lib.deno.d.ts

The declaration file could be saved and used for typing information.",
    )
}

fn upgrade_subcommand() -> Command {
  Command::new("upgrade")
    .about("Upgrade deno executable to given version")
    .long_about(
      "Upgrade deno executable to the given version.
Defaults to latest.

The version is downloaded from
https://github.com/denoland/deno/releases
and is used to replace the current executable.

If you want to not replace the current Deno executable but instead download an
update to a different location, use the --output flag

  deno upgrade --output $HOME/my_deno",
    )
    .hide(cfg!(not(feature = "upgrade")))
    .defer(|cmd| {
      cmd
        .arg(
          Arg::new("version")
            .long("version")
            .help("The version to upgrade to"),
        )
        .arg(
          Arg::new("output")
            .long("output")
            .help("The path to output the updated version to")
            // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
            .value_parser(value_parser!(PathBuf))
            .value_hint(ValueHint::FilePath),
        )
        .arg(
          Arg::new("dry-run")
            .long("dry-run")
            .help("Perform all checks without replacing old exe")
            .action(ArgAction::SetTrue),
        )
        .arg(
          Arg::new("force")
            .long("force")
            .short('f')
            .help("Replace current exe even if not out-of-date")
            .action(ArgAction::SetTrue),
        )
        .arg(
          Arg::new("canary")
            .long("canary")
            .help("Upgrade to canary builds")
            .action(ArgAction::SetTrue),
        )
        .arg(ca_file_arg())
    })
}

fn vendor_subcommand() -> Command {
  Command::new("vendor")
    .about("Vendor remote modules into a local directory")
    .long_about(
      "Vendor remote modules into a local directory.

Analyzes the provided modules along with their dependencies, downloads
remote modules to the output directory, and produces an import map that
maps remote specifiers to the downloaded files.

  deno vendor main.ts
  deno run --import-map vendor/import_map.json main.ts

Remote modules and multiple modules may also be specified:

  deno vendor main.ts test.deps.ts https://deno.land/std/path/mod.ts",
    )
    .defer(|cmd| {
      cmd
        .arg(
          Arg::new("specifiers")
            .num_args(1..)
            .action(ArgAction::Append)
            .required(true),
        )
        .arg(
          Arg::new("output")
            .long("output")
            .help("The directory to output the vendored modules to")
            // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
            .value_parser(value_parser!(PathBuf))
            .value_hint(ValueHint::DirPath),
        )
        .arg(
          Arg::new("force")
            .long("force")
            .short('f')
            .help("Forcefully overwrite conflicting files in existing output directory")
            .action(ArgAction::SetTrue),
        )
        .arg(no_config_arg())
        .arg(config_arg())
        .arg(import_map_arg())
        .arg(lock_arg())
        .arg(node_modules_dir_arg())
        .arg(vendor_arg())
        .arg(reload_arg())
        .arg(ca_file_arg())
    })
}

fn publish_subcommand() -> Command {
  Command::new("publish")
    .hide(true)
    .about("Unstable preview feature: Publish the current working directory's package or workspace")
    // TODO: .long_about()
    .defer(|cmd| {
      cmd.arg(
        Arg::new("token")
          .long("token")
          .help("The API token to use when publishing. If unset, interactive authentication is be used")
      )
      .arg(config_arg())
      .arg(no_config_arg())
      .arg(
        Arg::new("dry-run")
          .long("dry-run")
          .help("Prepare the package for publishing performing all checks and validations without uploading")
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("allow-slow-types")
          .long("allow-slow-types")
          .help("Allow publishing with slow types")
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("allow-dirty")
        .long("allow-dirty")
        .help("Allow publishing if the repository has uncommited changed")
        .action(ArgAction::SetTrue),
      ).arg(
        Arg::new("no-provenance")
          .long("no-provenance")
          .help("Disable provenance attestation. Enabled by default on Github actions, publicly links the package to where it was built and published from.")
          .action(ArgAction::SetTrue)
      )
      .arg(check_arg(/* type checks by default */ true))
      .arg(no_check_arg())
    })
}

fn compile_args(app: Command) -> Command {
  compile_args_without_check_args(app.arg(no_check_arg()))
}

fn compile_args_without_check_args(app: Command) -> Command {
  app
    .arg(import_map_arg())
    .arg(no_remote_arg())
    .arg(no_npm_arg())
    .arg(node_modules_dir_arg())
    .arg(vendor_arg())
    .arg(config_arg())
    .arg(no_config_arg())
    .arg(reload_arg())
    .arg(lock_arg())
    .arg(lock_write_arg())
    .arg(no_lock_arg())
    .arg(ca_file_arg())
}

static ALLOW_READ_HELP: &str = concat!(
  "Allow file system read access. Optionally specify allowed paths.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --allow-read\n",
  "  --allow-read=\"/etc,/var/log.txt\""
);

static DENY_READ_HELP: &str = concat!(
  "Deny file system read access. Optionally specify denied paths.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --deny-read\n",
  "  --deny-read=\"/etc,/var/log.txt\""
);

static ALLOW_WRITE_HELP: &str = concat!(
  "Allow file system write access. Optionally specify allowed paths.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --allow-write\n",
  "  --allow-write=\"/etc,/var/log.txt\""
);

static DENY_WRITE_HELP: &str = concat!(
  "Deny file system write access. Optionally specify denied paths.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --deny-write\n",
  "  --deny-write=\"/etc,/var/log.txt\""
);

static ALLOW_NET_HELP: &str = concat!(
  "Allow network access. Optionally specify allowed IP addresses and host names, with ports as necessary.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --allow-net\n",
  "  --allow-net=\"localhost:8080,deno.land\""
);

static DENY_NET_HELP: &str = concat!(
  "Deny network access. Optionally specify denied IP addresses and host names, with ports as necessary.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --deny-net\n",
  "  --deny-net=\"localhost:8080,deno.land\""
);

static ALLOW_ENV_HELP: &str = concat!(
  "Allow access to system environment information. Optionally specify accessible environment variables.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --allow-env\n",
  "  --allow-env=\"PORT,HOME,PATH\""
);

static DENY_ENV_HELP: &str = concat!(
  "Deny access to system environment information. Optionally specify accessible environment variables.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --deny-env\n",
  "  --deny-env=\"PORT,HOME,PATH\""
);

static ALLOW_SYS_HELP: &str = concat!(
  "Allow access to OS information. Optionally allow specific APIs by function name.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --allow-sys\n",
  "  --allow-sys=\"systemMemoryInfo,osRelease\""
);

static DENY_SYS_HELP: &str = concat!(
  "Deny access to OS information. Optionally deny specific APIs by function name.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --deny-sys\n",
  "  --deny-sys=\"systemMemoryInfo,osRelease\""
);

static ALLOW_RUN_HELP: &str = concat!(
  "Allow running subprocesses. Optionally specify allowed runnable program names.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --allow-run\n",
  "  --allow-run=\"whoami,ps\""
);

static DENY_RUN_HELP: &str = concat!(
  "Deny running subprocesses. Optionally specify denied runnable program names.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --deny-run\n",
  "  --deny-run=\"whoami,ps\""
);

static ALLOW_FFI_HELP: &str = concat!(
  "(Unstable) Allow loading dynamic libraries. Optionally specify allowed directories or files.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --allow-ffi\n",
  "  --allow-ffi=\"./libfoo.so\""
);

static DENY_FFI_HELP: &str = concat!(
  "(Unstable) Deny loading dynamic libraries. Optionally specify denied directories or files.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n",
  "Examples:\n",
  "  --deny-ffi\n",
  "  --deny-ffi=\"./libfoo.so\""
);

static ALLOW_HRTIME_HELP: &str = concat!(
  "Allow high-resolution time measurement. Note: this can enable timing attacks and fingerprinting.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n"
);

static DENY_HRTIME_HELP: &str = concat!(
  "Deny high-resolution time measurement. Note: this can prevent timing attacks and fingerprinting.\n",
  "Docs: https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n"
);

static ALLOW_ALL_HELP: &str = concat!(
  "Allow all permissions. Learn more about permissions in Deno:\n",
  "https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/basics/permissions\n"
);

fn permission_args(app: Command) -> Command {
  app
    .arg(
      Arg::new("allow-read")
        .long("allow-read")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("PATH")
        .help(ALLOW_READ_HELP)
        // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
        .value_parser(value_parser!(PathBuf))
        .value_hint(ValueHint::AnyPath),
    )
    .arg(
      Arg::new("deny-read")
        .long("deny-read")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("PATH")
        .help(DENY_READ_HELP)
        // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
        .value_parser(value_parser!(PathBuf))
        .value_hint(ValueHint::AnyPath),
    )
    .arg(
      Arg::new("allow-write")
        .long("allow-write")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("PATH")
        .help(ALLOW_WRITE_HELP)
        // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
        .value_parser(value_parser!(PathBuf))
        .value_hint(ValueHint::AnyPath),
    )
    .arg(
      Arg::new("deny-write")
        .long("deny-write")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("PATH")
        .help(DENY_WRITE_HELP)
        // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
        .value_parser(value_parser!(PathBuf))
        .value_hint(ValueHint::AnyPath),
    )
    .arg(
      Arg::new("allow-net")
        .long("allow-net")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("IP_OR_HOSTNAME")
        .help(ALLOW_NET_HELP)
        .value_parser(flags_net::validator),
    )
    .arg(
      Arg::new("deny-net")
        .long("deny-net")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("IP_OR_HOSTNAME")
        .help(DENY_NET_HELP)
        .value_parser(flags_net::validator),
    )
    .arg(unsafely_ignore_certificate_errors_arg())
    .arg(
      Arg::new("allow-env")
        .long("allow-env")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("VARIABLE_NAME")
        .help(ALLOW_ENV_HELP)
        .value_parser(|key: &str| {
          if key.is_empty() || key.contains(&['=', '\0'] as &[char]) {
            return Err(format!("invalid key \"{key}\""));
          }

          Ok(if cfg!(windows) {
            key.to_uppercase()
          } else {
            key.to_string()
          })
        }),
    )
    .arg(
      Arg::new("deny-env")
        .long("deny-env")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("VARIABLE_NAME")
        .help(DENY_ENV_HELP)
        .value_parser(|key: &str| {
          if key.is_empty() || key.contains(&['=', '\0'] as &[char]) {
            return Err(format!("invalid key \"{key}\""));
          }

          Ok(if cfg!(windows) {
            key.to_uppercase()
          } else {
            key.to_string()
          })
        }),
    )
    .arg(
      Arg::new("allow-sys")
        .long("allow-sys")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("API_NAME")
        .help(ALLOW_SYS_HELP)
        .value_parser(|key: &str| parse_sys_kind(key).map(ToString::to_string)),
    )
    .arg(
      Arg::new("deny-sys")
        .long("deny-sys")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("API_NAME")
        .help(DENY_SYS_HELP)
        .value_parser(|key: &str| parse_sys_kind(key).map(ToString::to_string)),
    )
    .arg(
      Arg::new("allow-run")
        .long("allow-run")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("PROGRAM_NAME")
        .help(ALLOW_RUN_HELP),
    )
    .arg(
      Arg::new("deny-run")
        .long("deny-run")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("PROGRAM_NAME")
        .help(DENY_RUN_HELP),
    )
    .arg(
      Arg::new("allow-ffi")
        .long("allow-ffi")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("PATH")
        .help(ALLOW_FFI_HELP)
        // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
        .value_parser(value_parser!(PathBuf))
        .value_hint(ValueHint::AnyPath),
    )
    .arg(
      Arg::new("deny-ffi")
        .long("deny-ffi")
        .num_args(0..)
        .use_value_delimiter(true)
        .require_equals(true)
        .value_name("PATH")
        .help(DENY_FFI_HELP)
        // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
        .value_parser(value_parser!(PathBuf))
        .value_hint(ValueHint::AnyPath),
    )
    .arg(
      Arg::new("allow-hrtime")
        .long("allow-hrtime")
        .action(ArgAction::SetTrue)
        .help(ALLOW_HRTIME_HELP),
    )
    .arg(
      Arg::new("deny-hrtime")
        .long("deny-hrtime")
        .action(ArgAction::SetTrue)
        .help(DENY_HRTIME_HELP),
    )
    .arg(
      Arg::new("allow-all")
        .short('A')
        .long("allow-all")
        .action(ArgAction::SetTrue)
        .help(ALLOW_ALL_HELP),
    )
    .arg(
      Arg::new("no-prompt")
        .long("no-prompt")
        .action(ArgAction::SetTrue)
        .help("Always throw if required permission wasn't passed"),
    )
}

fn runtime_args(
  app: Command,
  include_perms: bool,
  include_inspector: bool,
) -> Command {
  let app = compile_args(app);
  let app = if include_perms {
    permission_args(app)
  } else {
    app
  };
  let app = if include_inspector {
    inspect_args(app)
  } else {
    app
  };
  app
    .arg(cached_only_arg())
    .arg(location_arg())
    .arg(v8_flags_arg())
    .arg(seed_arg())
    .arg(enable_testing_features_arg())
    .arg(strace_ops_arg())
}

fn inspect_args(app: Command) -> Command {
  app
    .arg(
      Arg::new("inspect")
        .long("inspect")
        .value_name("HOST_AND_PORT")
        .help("Activate inspector on host:port (default: 127.0.0.1:9229)")
        .num_args(0..=1)
        .require_equals(true)
        .value_parser(value_parser!(SocketAddr)),
    )
    .arg(
      Arg::new("inspect-brk")
        .long("inspect-brk")
        .value_name("HOST_AND_PORT")
        .help(
          "Activate inspector on host:port, wait for debugger to connect and break at the start of user script",
        )
        .num_args(0..=1)
        .require_equals(true)
        .value_parser(value_parser!(SocketAddr)),
    )
    .arg(
      Arg::new("inspect-wait")
        .long("inspect-wait")
        .value_name("HOST_AND_PORT")
        .help(
          "Activate inspector on host:port and wait for debugger to connect before running user code",
        )
        .num_args(0..=1)
        .require_equals(true)
        .value_parser(value_parser!(SocketAddr)),
    )
}

static IMPORT_MAP_HELP: &str = concat!(
  "Load import map file from local file or remote URL.
Docs: https://docs.deno.com/runtime/manual/basics/import_maps
Specification: https://wicg.github.io/import-maps/
Examples: https://github.com/WICG/import-maps#the-import-map",
);

fn import_map_arg() -> Arg {
  Arg::new("import-map")
    .long("import-map")
    .alias("importmap")
    .value_name("FILE")
    .help("Load import map file")
    .long_help(IMPORT_MAP_HELP)
    .value_hint(ValueHint::FilePath)
}

fn env_file_arg() -> Arg {
  Arg::new("env")
    .long("env")
    .value_name("FILE")
    .help("Load .env file")
    .long_help("UNSTABLE: Load environment variables from local file. Only the first environment variable with a given key is used. Existing process environment variables are not overwritten.")
    .value_hint(ValueHint::FilePath)
    .default_missing_value(".env")
    .require_equals(true)
    .num_args(0..=1)
}

fn reload_arg() -> Arg {
  Arg::new("reload")
    .short('r')
    .num_args(0..)
    .use_value_delimiter(true)
    .require_equals(true)
    .long("reload")
    .help("Reload source code cache (recompile TypeScript)")
    .value_name("CACHE_BLOCKLIST")
    .long_help(
      "Reload source code cache (recompile TypeScript)
--reload
  Reload everything
--reload=https://deno.land/std
  Reload only standard modules
--reload=https://deno.land/std/fs/utils.ts,https://deno.land/std/fmt/colors.ts
  Reloads specific modules
--reload=npm:
  Reload all npm modules
--reload=npm:chalk
  Reload specific npm module",
    )
    .value_hint(ValueHint::FilePath)
    .value_parser(reload_arg_validate)
}

fn ca_file_arg() -> Arg {
  Arg::new("cert")
    .long("cert")
    .value_name("FILE")
    .help("Load certificate authority from PEM encoded file")
    .value_hint(ValueHint::FilePath)
}

fn cached_only_arg() -> Arg {
  Arg::new("cached-only")
    .long("cached-only")
    .action(ArgAction::SetTrue)
    .help("Require that remote dependencies are already cached")
}

/// Used for subcommands that operate on executable scripts only.
/// `deno fmt` has its own `--ext` arg because its possible values differ.
/// If --ext is not provided and the script doesn't have a file extension,
/// deno_graph::parse_module() defaults to js.
fn executable_ext_arg() -> Arg {
  Arg::new("ext")
    .long("ext")
    .help("Set content type of the supplied file")
    .value_parser(["ts", "tsx", "js", "jsx"])
}

fn location_arg() -> Arg {
  Arg::new("location")
    .long("location")
    .value_name("HREF")
    .value_parser(|href: &str| -> Result<Url, String> {
      let url = Url::parse(href);
      if url.is_err() {
        return Err("Failed to parse URL".to_string());
      }
      let mut url = url.unwrap();
      if !["http", "https"].contains(&url.scheme()) {
        return Err("Expected protocol \"http\" or \"https\"".to_string());
      }
      url.set_username("").unwrap();
      url.set_password(None).unwrap();
      Ok(url)
    })
    .help("Value of 'globalThis.location' used by some web APIs")
    .value_hint(ValueHint::Url)
}

fn enable_testing_features_arg() -> Arg {
  Arg::new("enable-testing-features-do-not-use")
    .long("enable-testing-features-do-not-use")
    .help("INTERNAL: Enable internal features used during integration testing")
    .action(ArgAction::SetTrue)
    .hide(true)
}

fn strace_ops_arg() -> Arg {
  Arg::new("strace-ops")
    .long("strace-ops")
    .num_args(0..)
    .use_value_delimiter(true)
    .require_equals(true)
    .value_name("OPS")
    .help("Trace low-level op calls")
    .hide(true)
}

fn v8_flags_arg() -> Arg {
  Arg::new("v8-flags")
    .long("v8-flags")
    .num_args(..)
    .use_value_delimiter(true)
    .require_equals(true)
    .help("Set V8 command line options")
    .long_help(
      "To see a list of all available flags use --v8-flags=--help.
    Any flags set with this flag are appended after the DENO_V8_FLAGS environmental variable",
    )
}

fn seed_arg() -> Arg {
  Arg::new("seed")
    .long("seed")
    .value_name("NUMBER")
    .help("Set the random number generator seed")
    .value_parser(value_parser!(u64))
}

fn hmr_arg(takes_files: bool) -> Arg {
  let arg = Arg::new("hmr")
    .long("unstable-hmr")
    .help("UNSTABLE: Watch for file changes and hot replace modules")
    .conflicts_with("watch");

  if takes_files {
    arg
      .value_name("FILES")
      .num_args(0..)
      // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
      .value_parser(value_parser!(PathBuf))
      .use_value_delimiter(true)
      .require_equals(true)
      .long_help(
        "Watch for file changes and restart process automatically.
Local files from entry point module graph are watched by default.
Additional paths might be watched by passing them as arguments to this flag.",
      )
      .value_hint(ValueHint::AnyPath)
  } else {
    arg.action(ArgAction::SetTrue).long_help(
      "Watch for file changes and restart process automatically.
      Only local files from entry point module graph are watched.",
    )
  }
}

fn watch_arg(takes_files: bool) -> Arg {
  let arg = Arg::new("watch")
    .long("watch")
    .help("Watch for file changes and restart automatically");

  if takes_files {
    arg
      .value_name("FILES")
      .num_args(0..)
      // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
      .value_parser(value_parser!(PathBuf))
      .use_value_delimiter(true)
      .require_equals(true)
      .long_help(
        "Watch for file changes and restart process automatically.
Local files from entry point module graph are watched by default.
Additional paths might be watched by passing them as arguments to this flag.",
      )
      .value_hint(ValueHint::AnyPath)
  } else {
    arg.action(ArgAction::SetTrue).long_help(
      "Watch for file changes and restart process automatically.
      Only local files from entry point module graph are watched.",
    )
  }
}

fn no_clear_screen_arg() -> Arg {
  Arg::new("no-clear-screen")
    .requires("watch")
    .long("no-clear-screen")
    .action(ArgAction::SetTrue)
    .help("Do not clear terminal screen when under watch mode")
}

fn no_check_arg() -> Arg {
  Arg::new("no-check")
    .num_args(0..=1)
    .require_equals(true)
    .value_name("NO_CHECK_TYPE")
    .long("no-check")
    .help("Skip type-checking modules")
    .long_help(
      "Skip type-checking. If the value of '--no-check=remote' is supplied,
diagnostic errors from remote modules will be ignored.",
    )
}

fn check_arg(checks_local_by_default: bool) -> Arg {
  let arg = Arg::new("check")
    .conflicts_with("no-check")
    .long("check")
    .num_args(0..=1)
    .require_equals(true)
    .value_name("CHECK_TYPE")
    .help("Type-check modules");

  if checks_local_by_default {
    arg.long_help(
      "Set type-checking behavior. This subcommand type-checks local modules by
default, so adding --check is redundant.
If the value of '--check=all' is supplied, diagnostic errors from remote modules
will be included.

Alternatively, the 'deno check' subcommand can be used.",
    )
  } else {
    arg.long_help(
      "Enable type-checking. This subcommand does not type-check by default.
If the value of '--check=all' is supplied, diagnostic errors from remote modules
will be included.

Alternatively, the 'deno check' subcommand can be used.",
    )
  }
}

fn script_arg() -> Arg {
  Arg::new("script_arg")
    .num_args(0..)
    .action(ArgAction::Append)
    // NOTE: these defaults are provided
    // so `deno run --v8-flags=--help` works
    // without specifying file to run.
    .default_value_ifs([
      ("v8-flags", "--help", Some("_")),
      ("v8-flags", "-help", Some("_")),
    ])
    .help("Script arg")
    .value_name("SCRIPT_ARG")
    .value_hint(ValueHint::FilePath)
}

fn lock_arg() -> Arg {
  Arg::new("lock")
    .long("lock")
    .value_name("FILE")
    .help(
      "Check the specified lock file.

If value is not provided, defaults to \"deno.lock\" in the current working directory.",
    )
    .num_args(0..=1)
    // todo(dsherret): remove value_parser!(PathBuf) and instead parse as string
    .value_parser(value_parser!(PathBuf))
    .value_hint(ValueHint::FilePath)
}

fn lock_write_arg() -> Arg {
  Arg::new("lock-write")
    .action(ArgAction::SetTrue)
    .long("lock-write")
    .help("Force overwriting the lock file.")
    .conflicts_with("no-lock")
}

fn no_lock_arg() -> Arg {
  Arg::new("no-lock")
    .long("no-lock")
    .action(ArgAction::SetTrue)
    .help("Disable auto discovery of the lock file.")
    .conflicts_with("lock")
}

static CONFIG_HELP: &str = concat!(
  "The configuration file can be used to configure different aspects of
deno including TypeScript, linting, and code formatting. Typically the
configuration file will be called `deno.json` or `deno.jsonc` and
automatically detected; in that case this flag is not necessary.
See https://deno.land/manual@v",
  "CARGO_PKG_VERSION",
  "/getting_started/configuration_file"
);

fn config_arg() -> Arg {
  Arg::new("config")
    .short('c')
    .long("config")
    .value_name("FILE")
    .help("Specify the configuration file")
    .long_help(CONFIG_HELP)
    .value_hint(ValueHint::FilePath)
}

fn no_config_arg() -> Arg {
  Arg::new("no-config")
    .long("no-config")
    .action(ArgAction::SetTrue)
    .help("Disable automatic loading of the configuration file.")
    .conflicts_with("config")
}

fn no_remote_arg() -> Arg {
  Arg::new("no-remote")
    .long("no-remote")
    .action(ArgAction::SetTrue)
    .help("Do not resolve remote modules")
}

fn no_npm_arg() -> Arg {
  Arg::new("no-npm")
    .long("no-npm")
    .action(ArgAction::SetTrue)
    .help("Do not resolve npm modules")
}

fn node_modules_dir_arg() -> Arg {
  Arg::new("node-modules-dir")
    .long("node-modules-dir")
    .num_args(0..=1)
    .value_parser(value_parser!(bool))
    .default_missing_value("true")
    .require_equals(true)
    .help("Enables or disables the use of a local node_modules folder for npm packages")
}

fn vendor_arg() -> Arg {
  Arg::new("vendor")
    .long("vendor")
    .num_args(0..=1)
    .value_parser(value_parser!(bool))
    .default_missing_value("true")
    .require_equals(true)
    .help("UNSTABLE: Enables or disables the use of a local vendor folder for remote modules and node_modules folder for npm packages")
}

fn unsafely_ignore_certificate_errors_arg() -> Arg {
  Arg::new("unsafely-ignore-certificate-errors")
    .long("unsafely-ignore-certificate-errors")
    .num_args(0..)
    .use_value_delimiter(true)
    .require_equals(true)
    .value_name("HOSTNAMES")
    .help("DANGER: Disables verification of TLS certificates")
    .value_parser(flags_net::validator)
}

fn add_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  let packages = matches.remove_many::<String>("packages").unwrap().collect();
  flags.subcommand = DenoSubcommand::Add(AddFlags { packages });
}

fn bench_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.type_check_mode = TypeCheckMode::Local;

  runtime_args_parse(flags, matches, true, false);

  // NOTE: `deno bench` always uses `--no-prompt`, tests shouldn't ever do
  // interactive prompts, unless done by user code
  flags.no_prompt = true;

  let json = matches.get_flag("json");

  let ignore = match matches.remove_many::<String>("ignore") {
    Some(f) => f.collect(),
    None => vec![],
  };

  let filter = matches.remove_one::<String>("filter");

  if matches.contains_id("script_arg") {
    flags
      .argv
      .extend(matches.remove_many::<String>("script_arg").unwrap());
  }

  let include = if let Some(files) = matches.remove_many::<String>("files") {
    files.collect()
  } else {
    Vec::new()
  };

  let no_run = matches.get_flag("no-run");

  flags.subcommand = DenoSubcommand::Bench(BenchFlags {
    files: FileFlags { include, ignore },
    filter,
    json,
    no_run,
    watch: watch_arg_parse(matches),
  });
}

fn bundle_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.type_check_mode = TypeCheckMode::Local;

  compile_args_parse(flags, matches);

  let source_file = matches.remove_one::<String>("source_file").unwrap();

  let out_file = if let Some(out_file) = matches.remove_one::<String>("out_file") {
    flags.allow_write = Some(vec![]);
    Some(out_file)
  } else {
    None
  };

  ext_arg_parse(flags, matches);

  flags.subcommand = DenoSubcommand::Bundle(BundleFlags {
    source_file,
    out_file,
    watch: watch_arg_parse(matches),
  });
}

fn cache_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  compile_args_parse(flags, matches);
  let files = matches.remove_many::<String>("file").unwrap().collect();
  flags.subcommand = DenoSubcommand::Cache(CacheFlags { files });
}

fn check_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.type_check_mode = TypeCheckMode::Local;
  compile_args_without_check_parse(flags, matches);
  let files = matches.remove_many::<String>("file").unwrap().collect();
  if matches.get_flag("all") || matches.get_flag("remote") {
    flags.type_check_mode = TypeCheckMode::All;
  }
  flags.subcommand = DenoSubcommand::Check(CheckFlags { files });
}

fn compile_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.type_check_mode = TypeCheckMode::Local;
  runtime_args_parse(flags, matches, true, false);

  let mut script = matches.remove_many::<String>("script_arg").unwrap();
  let source_file = script.next().unwrap();
  let args = script.collect();
  let output = matches.remove_one::<PathBuf>("output");
  let target = matches.remove_one::<String>("target");
  let no_terminal = matches.get_flag("no-terminal");
  let include = match matches.remove_many::<String>("include") {
    Some(f) => f.collect(),
    None => vec![],
  };
  ext_arg_parse(flags, matches);

  flags.subcommand = DenoSubcommand::Compile(CompileFlags {
    source_file,
    output,
    args,
    target,
    no_terminal,
    include,
  });
}

fn completions_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
  mut app: Command,
) {
  use clap_complete::generate;
  use clap_complete::shells::Bash;
  use clap_complete::shells::Fish;
  use clap_complete::shells::PowerShell;
  use clap_complete::shells::Zsh;
  use clap_complete_fig::Fig;

  let mut buf: Vec<u8> = vec![];
  let name = "deno";

  match matches.get_one::<String>("shell").unwrap().as_str() {
    "bash" => generate(Bash, &mut app, name, &mut buf),
    "fish" => generate(Fish, &mut app, name, &mut buf),
    "powershell" => generate(PowerShell, &mut app, name, &mut buf),
    "zsh" => generate(Zsh, &mut app, name, &mut buf),
    "fig" => generate(Fig, &mut app, name, &mut buf),
    _ => unreachable!(),
  }

  flags.subcommand = DenoSubcommand::Completions(CompletionsFlags {
    buf: buf.into_boxed_slice(),
  });
}

fn coverage_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  let files = match matches.remove_many::<String>("files") {
    Some(f) => f.collect(),
    None => vec!["coverage".to_string()], // default
  };
  let ignore = match matches.remove_many::<String>("ignore") {
    Some(f) => f.collect(),
    None => vec![],
  };
  let include = match matches.remove_many::<String>("include") {
    Some(f) => f.collect(),
    None => vec![],
  };
  let exclude = match matches.remove_many::<String>("exclude") {
    Some(f) => f.collect(),
    None => vec![],
  };
  let r#type = if matches.get_flag("lcov") {
    CoverageType::Lcov
  } else if matches.get_flag("html") {
    CoverageType::Html
  } else if matches.get_flag("detailed") {
    CoverageType::Detailed
  } else {
    CoverageType::Summary
  };
  let output = matches.remove_one::<PathBuf>("output");
  flags.subcommand = DenoSubcommand::Coverage(CoverageFlags {
    files: FileFlags {
      include: files,
      ignore,
    },
    output,
    include,
    exclude,
    r#type,
  });
}

fn doc_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  import_map_arg_parse(flags, matches);
  reload_arg_parse(flags, matches);
  lock_arg_parse(flags, matches);
  no_lock_arg_parse(flags, matches);
  no_npm_arg_parse(flags, matches);
  no_remote_arg_parse(flags, matches);

  let source_files_val = matches.remove_many::<String>("source_file");
  let source_files = if let Some(val) = source_files_val {
    let vals: Vec<String> = val.collect();

    if vals.len() == 1 {
      if vals[0] == "--builtin" {
        DocSourceFileFlag::Builtin
      } else {
        DocSourceFileFlag::Paths(vec![vals[0].to_string()])
      }
    } else {
      DocSourceFileFlag::Paths(vals.into_iter().filter(|v| v != "--builtin").collect())
    }
  } else {
    DocSourceFileFlag::Builtin
  };
  let private = matches.get_flag("private");
  let lint = matches.get_flag("lint");
  let json = matches.get_flag("json");
  let filter = matches.remove_one::<String>("filter");
  let html = if matches.get_flag("html") {
    let name = matches.remove_one::<String>("name").unwrap();
    let output = matches
      .remove_one::<PathBuf>("output")
      .unwrap_or(PathBuf::from("./docs/"));
    Some(DocHtmlFlag { name, output })
  } else {
    None
  };

  flags.subcommand = DenoSubcommand::Doc(DocFlags {
    source_files,
    json,
    lint,
    html,
    filter,
    private,
  });
}

fn eval_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  runtime_args_parse(flags, matches, false, true);
  flags.allow_net = Some(vec![]);
  flags.allow_env = Some(vec![]);
  flags.allow_run = Some(vec![]);
  flags.allow_read = Some(vec![]);
  flags.allow_sys = Some(vec![]);
  flags.allow_write = Some(vec![]);
  flags.allow_ffi = Some(vec![]);
  flags.allow_hrtime = true;

  ext_arg_parse(flags, matches);

  // TODO(@satyarohith): remove this flag in 2.0.
  let as_typescript = matches.get_flag("ts");

  if as_typescript {
    eprintln!(
      "⚠️ {}",
      crate::deno_cli::colors::yellow(
        "Use `--ext=ts` instead. The `--ts` and `-T` flags are deprecated and will be removed in Deno 2.0."
      ),
    );

    flags.ext = Some("ts".to_string());
  }

  let print = matches.get_flag("print");
  let mut code_args = matches.remove_many::<String>("code_arg").unwrap();
  let code = code_args.next().unwrap();
  flags.argv.extend(code_args);

  flags.subcommand = DenoSubcommand::Eval(EvalFlags { print, code });
}

fn fmt_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  config_args_parse(flags, matches);
  ext_arg_parse(flags, matches);

  let include = match matches.remove_many::<String>("files") {
    Some(f) => f.collect(),
    None => vec![],
  };
  let ignore = match matches.remove_many::<String>("ignore") {
    Some(f) => f.collect(),
    None => vec![],
  };

  let use_tabs = matches.remove_one::<bool>("use-tabs");
  let line_width = matches.remove_one::<NonZeroU32>("line-width");
  let indent_width = matches.remove_one::<NonZeroU8>("indent-width");
  let single_quote = matches.remove_one::<bool>("single-quote");
  let prose_wrap = matches.remove_one::<String>("prose-wrap");
  let no_semicolons = matches.remove_one::<bool>("no-semicolons");

  flags.subcommand = DenoSubcommand::Fmt(FmtFlags {
    check: matches.get_flag("check"),
    files: FileFlags { include, ignore },
    use_tabs,
    line_width,
    indent_width,
    single_quote,
    prose_wrap,
    no_semicolons,
    watch: watch_arg_parse(matches),
  });
}

fn init_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.subcommand = DenoSubcommand::Init(InitFlags {
    dir: matches.remove_one::<String>("dir"),
  });
}

fn info_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  reload_arg_parse(flags, matches);
  config_args_parse(flags, matches);
  import_map_arg_parse(flags, matches);
  location_arg_parse(flags, matches);
  ca_file_arg_parse(flags, matches);
  node_modules_and_vendor_dir_arg_parse(flags, matches);
  lock_args_parse(flags, matches);
  no_remote_arg_parse(flags, matches);
  no_npm_arg_parse(flags, matches);
  let json = matches.get_flag("json");
  flags.subcommand = DenoSubcommand::Info(InfoFlags {
    file: matches.remove_one::<String>("file"),
    json,
  });
}

fn install_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  runtime_args_parse(flags, matches, true, true);

  let root = matches.remove_one::<String>("root");

  let force = matches.get_flag("force");
  let name = matches.remove_one::<String>("name");
  let mut cmd_values = matches.remove_many::<String>("cmd").unwrap();

  let module_url = cmd_values.next().unwrap();
  let args = cmd_values.collect();

  flags.subcommand = DenoSubcommand::Install(InstallFlags {
    name,
    module_url,
    args,
    root,
    force,
  });
}

fn jupyter_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  let conn_file = matches.remove_one::<PathBuf>("conn");
  let kernel = matches.get_flag("kernel");
  let install = matches.get_flag("install");

  flags.subcommand = DenoSubcommand::Jupyter(JupyterFlags {
    install,
    kernel,
    conn_file,
  });
}

fn uninstall_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  let root = matches.remove_one::<String>("root");

  let name = matches.remove_one::<String>("name").unwrap();
  flags.subcommand = DenoSubcommand::Uninstall(UninstallFlags { name, root });
}

fn lsp_parse(
  flags: &mut Flags,
  _matches: &mut ArgMatches,
) {
  flags.subcommand = DenoSubcommand::Lsp;
}

fn lint_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  config_args_parse(flags, matches);
  let files = match matches.remove_many::<String>("files") {
    Some(f) => f.collect(),
    None => vec![],
  };
  let ignore = match matches.remove_many::<String>("ignore") {
    Some(f) => f.collect(),
    None => vec![],
  };
  let rules = matches.get_flag("rules");
  let maybe_rules_tags = matches
    .remove_many::<String>("rules-tags")
    .map(|f| f.collect());

  let maybe_rules_include = matches
    .remove_many::<String>("rules-include")
    .map(|f| f.collect());

  let maybe_rules_exclude = matches
    .remove_many::<String>("rules-exclude")
    .map(|f| f.collect());

  let json = matches.get_flag("json");
  let compact = matches.get_flag("compact");
  flags.subcommand = DenoSubcommand::Lint(LintFlags {
    files: FileFlags {
      include: files,
      ignore,
    },
    rules,
    maybe_rules_tags,
    maybe_rules_include,
    maybe_rules_exclude,
    json,
    compact,
    watch: watch_arg_parse(matches),
  });
}

fn repl_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  runtime_args_parse(flags, matches, true, true);
  unsafely_ignore_certificate_errors_parse(flags, matches);

  let eval_files = matches
    .remove_many::<String>("eval-file")
    .map(|values| values.collect());

  handle_repl_flags(
    flags,
    ReplFlags {
      eval_files,
      eval: matches.remove_one::<String>("eval"),
      is_default_command: false,
    },
  );
}

fn run_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
  app: Command,
) -> clap::error::Result<()> {
  runtime_args_parse(flags, matches, true, true);

  let mut script_arg = matches.remove_many::<String>("script_arg").ok_or_else(|| {
    let mut app = app;
    let subcommand = &mut app.find_subcommand_mut("run").unwrap();
    subcommand.error(
      clap::error::ErrorKind::MissingRequiredArgument,
      "[SCRIPT_ARG] may only be omitted with --v8-flags=--help",
    )
  })?;

  let script = script_arg.next().unwrap();
  flags.argv.extend(script_arg);

  ext_arg_parse(flags, matches);

  flags.subcommand = DenoSubcommand::Run(RunFlags {
    script,
    watch: watch_arg_parse_with_paths(matches),
  });

  Ok(())
}

fn task_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.config_flag = matches
    .remove_one::<String>("config")
    .map(ConfigFlag::Path)
    .unwrap_or(ConfigFlag::Discover);

  let mut task_flags = TaskFlags {
    cwd: matches.remove_one::<String>("cwd"),
    task: None,
  };

  if let Some((task, mut matches)) = matches.remove_subcommand() {
    task_flags.task = Some(task);

    flags.argv.extend(
      matches
        .remove_many::<std::ffi::OsString>("")
        .into_iter()
        .flatten()
        .filter_map(|arg| arg.into_string().ok()),
    );
  }

  flags.subcommand = DenoSubcommand::Task(task_flags);
}

fn test_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.type_check_mode = TypeCheckMode::Local;
  runtime_args_parse(flags, matches, true, true);
  // NOTE: `deno test` always uses `--no-prompt`, tests shouldn't ever do
  // interactive prompts, unless done by user code
  flags.no_prompt = true;

  let ignore = match matches.remove_many::<String>("ignore") {
    Some(f) => f.collect(),
    None => vec![],
  };

  let no_run = matches.get_flag("no-run");
  let trace_leaks = matches.get_flag("trace-ops") || matches.get_flag("trace-leaks");
  if trace_leaks && matches.get_flag("trace-ops") {
    // We can't change this to use the log crate because its not configured
    // yet at this point since the flags haven't been parsed. This flag is
    // deprecated though so it's not worth changing the code to use the log
    // crate here and this is only done for testing anyway.
    eprintln!(
      "⚠️ {}",
      crate::deno_cli::colors::yellow("The `--trace-ops` flag is deprecated and will be removed in Deno 2.0.\nUse the `--trace-leaks` flag instead."),
    );
  }
  let doc = matches.get_flag("doc");
  let allow_none = matches.get_flag("allow-none");
  let filter = matches.remove_one::<String>("filter");

  let fail_fast = if matches.contains_id("fail-fast") {
    Some(
      matches
        .remove_one::<NonZeroUsize>("fail-fast")
        .unwrap_or_else(|| NonZeroUsize::new(1).unwrap()),
    )
  } else {
    None
  };

  let shuffle = if matches.contains_id("shuffle") {
    Some(
      matches
        .remove_one::<u64>("shuffle")
        .unwrap_or_else(rand::random),
    )
  } else {
    None
  };

  if let Some(script_arg) = matches.remove_many::<String>("script_arg") {
    flags.argv.extend(script_arg);
  }

  let concurrent_jobs = if matches.get_flag("parallel") {
    if let Ok(value) = env::var("DENO_JOBS") {
      value.parse::<NonZeroUsize>().ok()
    } else {
      std::thread::available_parallelism().ok()
    }
  } else if matches.contains_id("jobs") {
    // We can't change this to use the log crate because its not configured
    // yet at this point since the flags haven't been parsed. This flag is
    // deprecated though so it's not worth changing the code to use the log
    // crate here and this is only done for testing anyway.
    eprintln!(
      "⚠️ {}",
      crate::deno_cli::colors::yellow("The `--jobs` flag is deprecated and will be removed in Deno 2.0.\nUse the `--parallel` flag with possibly the `DENO_JOBS` environment variable instead.\nLearn more at: https://docs.deno.com/runtime/manual/basics/env_variables"),
    );
    if let Some(value) = matches.remove_one::<NonZeroUsize>("jobs") {
      Some(value)
    } else {
      std::thread::available_parallelism().ok()
    }
  } else {
    None
  };

  let include = if let Some(files) = matches.remove_many::<String>("files") {
    files.collect()
  } else {
    Vec::new()
  };

  let junit_path = matches.remove_one::<String>("junit-path");

  let reporter = if let Some(reporter) = matches.remove_one::<String>("reporter") {
    match reporter.as_str() {
      "pretty" => TestReporterConfig::Pretty,
      "junit" => TestReporterConfig::Junit,
      "dot" => TestReporterConfig::Dot,
      "tap" => TestReporterConfig::Tap,
      _ => unreachable!(),
    }
  } else {
    TestReporterConfig::Pretty
  };

  if matches!(reporter, TestReporterConfig::Dot | TestReporterConfig::Tap) {
    flags.log_level = Some(Level::Error);
  }

  flags.subcommand = DenoSubcommand::Test(TestFlags {
    no_run,
    doc,
    coverage_dir: matches.remove_one::<String>("coverage"),
    fail_fast,
    files: FileFlags { include, ignore },
    filter,
    shuffle,
    allow_none,
    concurrent_jobs,
    trace_leaks,
    watch: watch_arg_parse(matches),
    reporter,
    junit_path,
  });
}

fn types_parse(
  flags: &mut Flags,
  _matches: &mut ArgMatches,
) {
  flags.subcommand = DenoSubcommand::Types;
}

fn upgrade_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  ca_file_arg_parse(flags, matches);

  let dry_run = matches.get_flag("dry-run");
  let force = matches.get_flag("force");
  let canary = matches.get_flag("canary");
  let version = matches.remove_one::<String>("version");
  let output = matches.remove_one::<PathBuf>("output");
  flags.subcommand = DenoSubcommand::Upgrade(UpgradeFlags {
    dry_run,
    force,
    canary,
    version,
    output,
  });
}

fn vendor_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  ca_file_arg_parse(flags, matches);
  config_args_parse(flags, matches);
  import_map_arg_parse(flags, matches);
  lock_arg_parse(flags, matches);
  node_modules_and_vendor_dir_arg_parse(flags, matches);
  reload_arg_parse(flags, matches);

  flags.subcommand = DenoSubcommand::Vendor(VendorFlags {
    specifiers: matches
      .remove_many::<String>("specifiers")
      .map(|p| p.collect())
      .unwrap_or_default(),
    output_path: matches.remove_one::<PathBuf>("output"),
    force: matches.get_flag("force"),
  });
}

fn publish_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.type_check_mode = TypeCheckMode::Local; // local by default
  no_check_arg_parse(flags, matches);
  check_arg_parse(flags, matches);
  config_args_parse(flags, matches);

  flags.subcommand = DenoSubcommand::Publish(PublishFlags {
    token: matches.remove_one("token"),
    dry_run: matches.get_flag("dry-run"),
    allow_slow_types: matches.get_flag("allow-slow-types"),
    allow_dirty: matches.get_flag("allow-dirty"),
    no_provenance: matches.get_flag("no-provenance"),
  });
}

fn compile_args_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  compile_args_without_check_parse(flags, matches);
  no_check_arg_parse(flags, matches);
  check_arg_parse(flags, matches);
}

fn compile_args_without_check_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  import_map_arg_parse(flags, matches);
  no_remote_arg_parse(flags, matches);
  no_npm_arg_parse(flags, matches);
  node_modules_and_vendor_dir_arg_parse(flags, matches);
  config_args_parse(flags, matches);
  reload_arg_parse(flags, matches);
  lock_args_parse(flags, matches);
  ca_file_arg_parse(flags, matches);
}

fn permission_args_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  unsafely_ignore_certificate_errors_parse(flags, matches);
  if let Some(read_wl) = matches.remove_many::<PathBuf>("allow-read") {
    flags.allow_read = Some(read_wl.collect());
  }

  if let Some(read_wl) = matches.remove_many::<PathBuf>("deny-read") {
    flags.deny_read = Some(read_wl.collect());
  }

  if let Some(write_wl) = matches.remove_many::<PathBuf>("allow-write") {
    flags.allow_write = Some(write_wl.collect());
  }

  if let Some(write_wl) = matches.remove_many::<PathBuf>("deny-write") {
    flags.deny_write = Some(write_wl.collect());
  }

  if let Some(net_wl) = matches.remove_many::<String>("allow-net") {
    let net_allowlist = flags_net::parse(net_wl.collect()).unwrap();
    flags.allow_net = Some(net_allowlist);
  }

  if let Some(net_wl) = matches.remove_many::<String>("deny-net") {
    let net_denylist = flags_net::parse(net_wl.collect()).unwrap();
    flags.deny_net = Some(net_denylist);
  }

  if let Some(env_wl) = matches.remove_many::<String>("allow-env") {
    flags.allow_env = Some(env_wl.collect());
    debug!("env allowlist: {:#?}", &flags.allow_env);
  }

  if let Some(env_wl) = matches.remove_many::<String>("deny-env") {
    flags.deny_env = Some(env_wl.collect());
    debug!("env denylist: {:#?}", &flags.deny_env);
  }

  if let Some(run_wl) = matches.remove_many::<String>("allow-run") {
    flags.allow_run = Some(run_wl.collect());
    debug!("run allowlist: {:#?}", &flags.allow_run);
  }

  if let Some(run_wl) = matches.remove_many::<String>("deny-run") {
    flags.deny_run = Some(run_wl.collect());
    debug!("run denylist: {:#?}", &flags.deny_run);
  }

  if let Some(sys_wl) = matches.remove_many::<String>("allow-sys") {
    flags.allow_sys = Some(sys_wl.collect());
    debug!("sys info allowlist: {:#?}", &flags.allow_sys);
  }

  if let Some(sys_wl) = matches.remove_many::<String>("deny-sys") {
    flags.deny_sys = Some(sys_wl.collect());
    debug!("sys info denylist: {:#?}", &flags.deny_sys);
  }

  if let Some(ffi_wl) = matches.remove_many::<PathBuf>("allow-ffi") {
    flags.allow_ffi = Some(ffi_wl.collect());
    debug!("ffi allowlist: {:#?}", &flags.allow_ffi);
  }

  if let Some(ffi_wl) = matches.remove_many::<PathBuf>("deny-ffi") {
    flags.deny_ffi = Some(ffi_wl.collect());
    debug!("ffi denylist: {:#?}", &flags.deny_ffi);
  }

  if matches.get_flag("allow-hrtime") {
    flags.allow_hrtime = true;
  }

  if matches.get_flag("deny-hrtime") {
    flags.deny_hrtime = true;
  }

  if matches.get_flag("allow-all") {
    flags.allow_all = true;
    flags.allow_read = Some(vec![]);
    flags.allow_env = Some(vec![]);
    flags.allow_net = Some(vec![]);
    flags.allow_run = Some(vec![]);
    flags.allow_write = Some(vec![]);
    flags.allow_sys = Some(vec![]);
    flags.allow_ffi = Some(vec![]);
    flags.allow_hrtime = true;
  }

  if matches.get_flag("no-prompt") {
    flags.no_prompt = true;
  }
}

fn unsafely_ignore_certificate_errors_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  if let Some(ic_wl) = matches.remove_many::<String>("unsafely-ignore-certificate-errors") {
    let ic_allowlist = flags_net::parse(ic_wl.collect()).unwrap();
    flags.unsafely_ignore_certificate_errors = Some(ic_allowlist);
  }
}

fn runtime_args_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
  include_perms: bool,
  include_inspector: bool,
) {
  compile_args_parse(flags, matches);
  cached_only_arg_parse(flags, matches);
  if include_perms {
    permission_args_parse(flags, matches);
  }
  if include_inspector {
    inspect_arg_parse(flags, matches);
  }
  location_arg_parse(flags, matches);
  v8_flags_arg_parse(flags, matches);
  seed_arg_parse(flags, matches);
  enable_testing_features_arg_parse(flags, matches);
  env_file_arg_parse(flags, matches);
  strace_ops_parse(flags, matches);
}

fn inspect_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  let default = || "127.0.0.1:9229".parse::<SocketAddr>().unwrap();
  flags.inspect = if matches.contains_id("inspect") {
    Some(
      matches
        .remove_one::<SocketAddr>("inspect")
        .unwrap_or_else(default),
    )
  } else {
    None
  };
  flags.inspect_brk = if matches.contains_id("inspect-brk") {
    Some(
      matches
        .remove_one::<SocketAddr>("inspect-brk")
        .unwrap_or_else(default),
    )
  } else {
    None
  };
  flags.inspect_wait = if matches.contains_id("inspect-wait") {
    Some(
      matches
        .remove_one::<SocketAddr>("inspect-wait")
        .unwrap_or_else(default),
    )
  } else {
    None
  };
}

fn import_map_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.import_map_path = matches.remove_one::<String>("import-map");
}

fn env_file_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.env_file = matches.remove_one::<String>("env");
}

fn reload_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  if let Some(cache_bl) = matches.remove_many::<String>("reload") {
    let raw_cache_blocklist: Vec<String> = cache_bl.collect();
    if raw_cache_blocklist.is_empty() {
      flags.reload = true;
    } else {
      flags.cache_blocklist = resolve_urls(raw_cache_blocklist);
      debug!("cache blocklist: {:#?}", &flags.cache_blocklist);
      flags.reload = false;
    }
  }
}

fn ca_file_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.ca_data = matches.remove_one::<String>("cert").map(CaData::File);
}

fn enable_testing_features_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  if matches.get_flag("enable-testing-features-do-not-use") {
    flags.enable_testing_features = true
  }
}

fn strace_ops_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  if let Some(patterns) = matches.remove_many::<String>("strace-ops") {
    flags.strace_ops = Some(patterns.collect());
  }
}

fn cached_only_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  if matches.get_flag("cached-only") {
    flags.cached_only = true;
  }
}

fn ext_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.ext = matches.remove_one::<String>("ext");
}

fn location_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.location = matches.remove_one::<Url>("location");
}

fn v8_flags_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  if let Some(v8_flags) = matches.remove_many::<String>("v8-flags") {
    flags.v8_flags = v8_flags.collect();
  }
}

fn seed_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  if let Some(seed) = matches.remove_one::<u64>("seed") {
    flags.seed = Some(seed);

    flags.v8_flags.push(format!("--random-seed={seed}"));
  }
}

fn no_check_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  if let Some(cache_type) = matches.get_one::<String>("no-check") {
    match cache_type.as_str() {
      "remote" => flags.type_check_mode = TypeCheckMode::Local,
      _ => debug!(
        "invalid value for 'no-check' of '{}' using default",
        cache_type
      ),
    }
  } else if matches.contains_id("no-check") {
    flags.type_check_mode = TypeCheckMode::None;
  }
}

fn check_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  if let Some(cache_type) = matches.get_one::<String>("check") {
    match cache_type.as_str() {
      "all" => flags.type_check_mode = TypeCheckMode::All,
      _ => debug!(
        "invalid value for 'check' of '{}' using default",
        cache_type
      ),
    }
  } else if matches.contains_id("check") {
    flags.type_check_mode = TypeCheckMode::Local;
  }
}

fn lock_args_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  lock_arg_parse(flags, matches);
  no_lock_arg_parse(flags, matches);
  if matches.get_flag("lock-write") {
    flags.lock_write = true;
  }
}

fn lock_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  if matches.contains_id("lock") {
    let lockfile = matches
      .remove_one::<PathBuf>("lock")
      .unwrap_or_else(|| PathBuf::from("./deno.lock"));
    flags.lock = Some(lockfile);
  }
}

fn no_lock_arg_parse(
  flags: &mut Flags,
  matches: &ArgMatches,
) {
  if matches.get_flag("no-lock") {
    flags.no_lock = true;
  }
}

fn config_args_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.config_flag = if matches.get_flag("no-config") {
    ConfigFlag::Disabled
  } else if let Some(config) = matches.remove_one::<String>("config") {
    ConfigFlag::Path(config)
  } else {
    ConfigFlag::Discover
  };
}

fn no_remote_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  if matches.get_flag("no-remote") {
    flags.no_remote = true;
  }
}

fn no_npm_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  if matches.get_flag("no-npm") {
    flags.no_npm = true;
  }
}

fn node_modules_and_vendor_dir_arg_parse(
  flags: &mut Flags,
  matches: &mut ArgMatches,
) {
  flags.node_modules_dir = matches.remove_one::<bool>("node-modules-dir");
  flags.vendor = matches.remove_one::<bool>("vendor");
}

fn reload_arg_validate(urlstr: &str) -> Result<String, String> {
  if urlstr.is_empty() {
    return Err(String::from("Missing url. Check for extra commas."));
  }
  match Url::from_str(urlstr) {
    Ok(_) => Ok(urlstr.to_string()),
    Err(e) => Err(e.to_string()),
  }
}

fn watch_arg_parse(matches: &mut ArgMatches) -> Option<WatchFlags> {
  if matches.get_flag("watch") {
    Some(WatchFlags {
      hmr: false,
      no_clear_screen: matches.get_flag("no-clear-screen"),
    })
  } else {
    None
  }
}

fn watch_arg_parse_with_paths(matches: &mut ArgMatches) -> Option<WatchFlagsWithPaths> {
  if let Some(paths) = matches.remove_many::<PathBuf>("watch") {
    return Some(WatchFlagsWithPaths {
      paths: paths.collect(),
      hmr: false,
      no_clear_screen: matches.get_flag("no-clear-screen"),
    });
  }

  matches
    .remove_many::<PathBuf>("hmr")
    .map(|paths| WatchFlagsWithPaths {
      paths: paths.collect(),
      hmr: true,
      no_clear_screen: matches.get_flag("no-clear-screen"),
    })
}

// TODO(ry) move this to utility module and add test.
/// Strips fragment part of URL. Panics on bad URL.
pub fn resolve_urls(urls: Vec<String>) -> Vec<String> {
  let mut out: Vec<String> = vec![];
  for urlstr in urls.iter() {
    if let Ok(mut url) = Url::from_str(urlstr) {
      url.set_fragment(None);
      let mut full_url = String::from(url.as_str());
      if full_url.len() > 1 && full_url.ends_with('/') {
        full_url.pop();
      }
      out.push(full_url);
    } else {
      panic!("Bad Url: {urlstr}");
    }
  }
  out
}
