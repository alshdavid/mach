use std::path::PathBuf;

use crate::core::plugins::PluginContainer;
use crate::types::Dependency;
use crate::types::MachConfig;

#[derive(Debug)]
pub struct RunResolversResult {
  pub file_path: PathBuf,
  pub file_path_relative: PathBuf,
}

pub fn run_resolvers(
  config: &MachConfig,
  plugins: &PluginContainer,
  dependency: &Dependency,
) -> anyhow::Result<RunResolversResult> {
  for resolver in &plugins.resolvers {
    let Some(resolve_result) = resolver.resolve(&dependency)? else {
      continue;
    };

    return Ok(RunResolversResult {
      file_path_relative: pathdiff::diff_paths(&resolve_result.file_path, &config.project_root)
        .unwrap(),
      file_path: resolve_result.file_path,
    });
  }

  Err(anyhow::anyhow!(
    "Unable to resolve file: \n\tSpecifier: {}\n\tFrom: {:?}",
    dependency.specifier,
    dependency.resolve_from
  ))
}
