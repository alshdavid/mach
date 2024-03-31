use deno_core::error::AnyError;
use deno_runtime::permissions::PermissionsContainer;

use crate::deno_cli::args::{DenoSubcommand, Flags, RunFlags, UnstableConfig};
use crate::deno_cli::CliFactory;
use super::DenoInitOptions;

pub async fn run_script(options: DenoInitOptions) -> Result<i32, AnyError> {
  let subcommand = DenoSubcommand::Run(RunFlags {
    script: options.script.clone(),
    watch: None,
  });

  let flags = Flags{
    subcommand: subcommand.clone(),
    allow_all: true,
    unstable_config: UnstableConfig {
      byonm: true,
      ..Default::default()
    },
    ..Default::default()
  };

  // dbg!(&flags);

  let factory = CliFactory::from_flags(flags).await?;
  // let deno_dir = factory.deno_dir()?;
  // let http_client = factory.http_client();
  let cli_options = factory.cli_options();

  let main_module = cli_options.resolve_main_module()?;

  maybe_npm_install(&factory).await?;

  let permissions = PermissionsContainer::allow_all();

  let worker_factory = factory.create_cli_main_worker_factory().await?;

  let mut worker = worker_factory
    .create_custom_worker(
      main_module,
      permissions,
      options.extensions,
      options.stdio,
    )
    .await?;

  let exit_code = worker.run().await?;
  Ok(exit_code)
}

async fn maybe_npm_install(factory: &CliFactory) -> Result<(), AnyError> {
  // ensure an "npm install" is done if the user has explicitly
  // opted into using a managed node_modules directory
  if factory.cli_options().node_modules_dir_enablement() == Some(true) {
    if let Some(npm_resolver) = factory.npm_resolver().await?.as_managed() {
      npm_resolver.ensure_top_level_package_json_install().await?;
    }
  }
  Ok(())
}
