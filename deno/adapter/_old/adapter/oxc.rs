use normalize_path::NormalizePath;
use std::path::Path;
use std::path::PathBuf;

pub fn resolve_oxc(
  specifier: &str,
  from: &Path,
) -> Result<PathBuf, String> {
  let options = oxc_resolver::ResolveOptions {
    alias_fields: vec![],
    alias: vec![],
    ..oxc_resolver::ResolveOptions::default()
  };
  let resolver = oxc_resolver::Resolver::new(options);
  match resolver.resolve(from, specifier) {
    Err(error) => return Err(format!("{error}")),
    Ok(resolution) => return Ok(resolution.full_path().normalize()),
  };
}
