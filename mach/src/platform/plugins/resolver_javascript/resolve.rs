// TODO not a plugin yet

use std::path::Path;
use std::path::PathBuf;
use std::path::MAIN_SEPARATOR_STR;

use normalize_path::NormalizePath;
use oxc_resolver::ResolveOptions;
use oxc_resolver::Resolver;

pub fn resolve(
  from_raw: &PathBuf,
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

  let result = resolve_oxc(&specifier, &from);

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
    let result = resolve_oxc(&spec, &from);
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

fn resolve_oxc(
  specifier: &str,
  from: &Path,
) -> Result<PathBuf, String> {
  let options = ResolveOptions {
    alias_fields: vec![],
    alias: vec![],
    ..ResolveOptions::default()
  };

  let resolver = Resolver::new(options);

  match resolver.resolve(from, specifier) {
    Err(error) => return Err(format!("{error}")),
    Ok(resolution) => return Ok(resolution.full_path().normalize()),
  };
}
