#![allow(unused, dead_code, deprecated)]

use deno_terminal::colors;

pub mod args;
pub mod auth_tokens;
pub mod cache;
pub mod cdp;
pub mod deno_std;
pub mod emit;
pub mod errors;
pub mod factory;
pub mod file_fetcher;
pub mod graph_util;
pub mod http_util;
pub mod js;
pub mod jsr;
pub mod lsp;
pub mod module_loader;
pub mod node;
pub mod npm;
pub mod ops;
pub mod resolver;
pub mod standalone;
pub mod tools;
pub mod tsc;
pub mod util;
pub mod version;
pub mod worker;

pub use factory::CliFactory;

pub use deno_runtime::UNSTABLE_GRANULAR_FLAGS;

pub(crate) fn unstable_exit_cb(
  feature: &str,
  api_name: &str,
) {
  eprintln!(
    "Unstable API '{api_name}'. The `--unstable-{}` flag must be provided.",
    feature
  );
  std::process::exit(70);
}

// TODO(bartlomieju): remove when `--unstable` flag is removed.
pub(crate) fn unstable_warn_cb(
  feature: &str,
  api_name: &str,
) {
  eprintln!(
    "⚠️  {}",
    deno_terminal::colors::yellow(format!(
      "The `{}` API was used with `--unstable` flag. The `--unstable` flag is deprecated and will be removed in Deno 2.0. Use granular `--unstable-{}` instead.\nLearn more at: https://docs.deno.com/runtime/manual/tools/unstable_flags",
      api_name, feature
    ))
  );
}
