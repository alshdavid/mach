use std::path::PathBuf;

use crate::platform::config::PluginContainerSync;
use crate::public::Dependency;
use crate::public::MachConfig;

#[derive(Debug)]
pub struct RunResolversResult {
  pub file_path: PathBuf,
  pub file_path_relative: PathBuf,
}

pub fn run_resolvers(
  config: &MachConfig,
  plugins: &PluginContainerSync,
  dependency: &Dependency,
) -> Result<RunResolversResult, String> {
  for resolver in &plugins.resolvers {
    if let Some(resolve_result) = resolver.resolve(&dependency)? {
      return Ok(RunResolversResult {
        file_path_relative: pathdiff::diff_paths(&resolve_result.file_path, &config.project_root)
          .unwrap(),
        file_path: resolve_result.file_path,
      });
    }
  }
  return Err(format!(
    "Unable to resolve file: \n\tSpecifier: {}\n\tFrom: {:?}",
    dependency.specifier, dependency.resolve_from
  ));
}
