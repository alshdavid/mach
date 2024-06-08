// TODO not a plugin yet

use std::path::Path;
use std::path::PathBuf;
use std::path::MAIN_SEPARATOR_STR;

use normalize_path::NormalizePath;
use oxc_resolver::ResolveOptions;
use oxc_resolver::Resolver;

pub fn resolve(
  from_raw: &Path,
  specifier: &str,
) -> Result<PathBuf, String> {
  if specifier.starts_with(MAIN_SEPARATOR_STR) {
    return Ok(PathBuf::from(specifier).normalize());
  }

  let from = {
    if from_raw.is_file() {
      from_raw.parent().unwrap()
    } else {
      from_raw
    }
  };

  if specifier.starts_with(".") {
    return Ok(from.join(specifier).normalize());
  }

  let oxc_options = ResolveOptions {
    alias_fields: vec![],
    alias: vec![],
    ..ResolveOptions::default()
  };

  let result = resolve_oxc(&from, &specifier, oxc_options.clone());

  if let Ok(result) = result {
    return Ok(result);
  }

  // Ignore this, this is just some hacky stuff
  // for a personal project that will be removed
  for try_this in [
    ".js",
    ".jsx",
    ".ts",
    ".tsx",
    "/index.js",
    "/index.jsx",
    "/index.ts",
    "/index.tsx",
    "/src/index.js",
    "/src/index.jsx",
    "/src/index.ts",
    "/src/index.tsx",
  ] {
    let spec = format!("{}{}", specifier, try_this);
    let result = resolve_oxc(&from, &spec, oxc_options.clone());
    if let Ok(result) = result {
      return Ok(result);
    }
  }

  return Err(format!(
    "Unable to resolve:\n\tSpecifier: {}\n\tFrom:      {}",
    specifier,
    from_raw.to_str().unwrap(),
  ));
}

pub fn resolve_oxc(
  from: &Path,
  specifier: &str,
  options: ResolveOptions,
) -> Result<PathBuf, String> {
  let resolver = Resolver::new(options);

  match resolver.resolve(from, specifier) {
    Err(error) => return Err(format!("{error}")),
    Ok(resolution) => return Ok(resolution.full_path().normalize()),
  };
}
