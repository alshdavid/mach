use std::path::PathBuf;

use crate::types::Compilation;
use crate::types::Dependency;

#[derive(Debug)]
pub struct RunResolversResult {
  pub file_path: PathBuf,
  pub file_path_relative: PathBuf,
}

pub fn run_resolvers(
  c: &Compilation,
  dependency: &Dependency,
) -> anyhow::Result<RunResolversResult> {
  for resolver in &c.plugins.resolvers {
    let Some(resolve_result) = resolver.resolve(&dependency)? else {
      continue;
    };

    return Ok(RunResolversResult {
      file_path_relative: pathdiff::diff_paths(&resolve_result.file_path, &c.config.project_root)
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
