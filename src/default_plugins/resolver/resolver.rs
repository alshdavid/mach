// TODO not a plugin yet

use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use oxc_resolver::ResolveOptions;
use oxc_resolver::Resolver;

pub fn resolve(from_raw: &PathBuf, specifier: &str) -> Result<PathBuf, String> {
  let from = {
    if from_raw.is_file() {
      from_raw.parent().unwrap()
    } else {
      from_raw
    }
  };

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
    "Unable to resolve: \n\t{}\n\t{}",
    from_raw.to_str().unwrap(),
    specifier
  ));
}

fn resolve_oxc(specifier: &str, from: &Path) -> Result<PathBuf, String> {
  let options = ResolveOptions {
    alias_fields: vec![],
    alias: vec![],
    ..ResolveOptions::default()
  };

  let resolver = Resolver::new(options);

  match resolver.resolve(from, specifier) {
    Err(error) => return Err(format!("{error}")),
    Ok(resolution) => return Ok(normalize_path(&resolution.full_path())),
  };
}

pub fn normalize_path(path: &Path) -> PathBuf {
  let mut components = path.components().peekable();
  let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
    components.next();
    PathBuf::from(c.as_os_str())
  } else {
    PathBuf::new()
  };

  for component in components {
    match component {
      Component::Prefix(..) => unreachable!(),
      Component::RootDir => {
        ret.push(component.as_os_str());
      }
      Component::CurDir => {}
      Component::ParentDir => {
        ret.pop();
      }
      Component::Normal(c) => {
        ret.push(c);
      }
    }
  }
  return ret;
}
