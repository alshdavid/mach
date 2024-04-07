use deno_core::error::AnyError;
use deno_runtime::permissions::PermissionsContainer;

use super::DenoInitOptions;
use crate::deno_cli::args::DenoSubcommand;
use crate::deno_cli::args::Flags;
use crate::deno_cli::args::RunFlags;
use crate::deno_cli::args::UnstableConfig;
use crate::deno_cli::worker::CliMainWorker;
use crate::deno_cli::CliFactory;

pub async fn run_script(options: DenoInitOptions) -> Result<CliMainWorker, AnyError> {
  let subcommand = DenoSubcommand::Run(RunFlags {
    script: options.script.clone(),
    watch: None,
  });

  let flags = Flags {
    argv: options.args,
    subcommand: subcommand.clone(),
    allow_all: true,
    unstable_config: UnstableConfig {
      byonm: true,
      bare_node_builtins: true,
      ..Default::default()
    },
    ..Default::default()
  };

  let factory = CliFactory::from_flags(flags).await?;
  let cli_options = factory.cli_options();

  let main_module = cli_options.resolve_main_module()?;

  // maybe_npm_install(&factory).await?;

  let permissions = PermissionsContainer::allow_all();

  let worker_factory = factory.create_cli_main_worker_factory().await?;

  let worker = worker_factory
    .create_custom_worker(main_module, permissions, vec![], options.stdio)
    .await?;

  // let exit_code = worker.run().await?;
  Ok(worker)
}
