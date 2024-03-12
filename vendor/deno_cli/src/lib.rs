use deno_terminal::colors;

mod args;
mod cache;
mod lsp;
mod npm;
mod ops;
mod standalone;
mod tools;
mod tsc;
mod util;
mod auth_tokens;
mod cdp;
mod deno_std;
mod emit;
mod errors;
mod factory;
mod file_fetcher;
mod graph_util;
mod http_util;
mod js;
mod jsr;
mod module_loader;
mod node;
mod resolver;
mod version;
mod worker;
mod snapshots;

pub use factory::CliFactory;

pub use deno_runtime::UNSTABLE_GRANULAR_FLAGS;

pub(crate) fn unstable_exit_cb(feature: &str, api_name: &str) {
  eprintln!(
    "Unstable API '{api_name}'. The `--unstable-{}` flag must be provided.",
    feature
  );
  std::process::exit(70);
}

// TODO(bartlomieju): remove when `--unstable` flag is removed.
pub(crate) fn unstable_warn_cb(feature: &str, api_name: &str) {
  eprintln!(
    "⚠️  {}",
    deno_terminal::colors::yellow(format!(
      "The `{}` API was used with `--unstable` flag. The `--unstable` flag is deprecated and will be removed in Deno 2.0. Use granular `--unstable-{}` instead.\nLearn more at: https://docs.deno.com/runtime/manual/tools/unstable_flags",
      api_name, feature
    ))
  );
}
