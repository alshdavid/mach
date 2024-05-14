use normalize_path::NormalizePath;
use oxc_resolver::ResolveOptions as OxcResolveOptions;
use oxc_resolver::Resolver as OxcResolver;

use crate::public::Dependency;
use crate::public::ResolveResult;
use crate::public::Resolver;

#[derive(Debug)]
pub struct ResolverJavaScript {
  resolver: OxcResolver,
}

impl ResolverJavaScript {
  pub fn new() -> Self {
    let oxc_options = OxcResolveOptions {
      alias_fields: vec![],
      alias: vec![],
      exports_fields: vec![vec!["exports".to_string()]],
      condition_names: vec![
        "import".to_string(),
        "require".to_string(),
        "module".to_string(),
        "webpack".to_string(),
        "development".to_string(),
        "browser".to_string(), 
        "default".to_string(),
      ],
      ..OxcResolveOptions::default()
    };

    let resolver = OxcResolver::new(oxc_options);

    Self { 
      resolver,
    }
  }
}

impl Resolver for ResolverJavaScript {
  fn resolve(
    &self,
    dependency: &Dependency,
  ) -> Result<Option<ResolveResult>, String> {
    let resolve_from = {
      if dependency.resolve_from.is_file() {
        dependency.resolve_from.parent().unwrap()
      } else {
        &dependency.resolve_from
      }
    };

    if let Ok(resolution) = self.resolver.resolve(&resolve_from, &dependency.specifier) {
      return Ok(Some(ResolveResult {
        file_path: resolution.full_path().normalize(),
      }));
    };

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
      let specifier = format!("{}{}", dependency.specifier, try_this);
      if let Ok(resolution) = self.resolver.resolve(&resolve_from, &specifier) {
        return Ok(Some(ResolveResult {
          file_path: resolution.full_path().normalize(),
        }));
      };
    }

    return Ok(None);
  }
}
