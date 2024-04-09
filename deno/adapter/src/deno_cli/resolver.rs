// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use deno_ast::MediaType;
use deno_core::anyhow::anyhow;
use deno_core::anyhow::Context;
use deno_core::error::AnyError;
use deno_core::futures::future;
use deno_core::futures::future::LocalBoxFuture;
use deno_core::futures::FutureExt;
use deno_core::parking_lot::Mutex;
use deno_core::ModuleCodeString;
use deno_core::ModuleSpecifier;
use deno_graph::source::NpmPackageReqResolution;
use deno_graph::source::NpmResolver;
use deno_graph::source::ResolutionMode;
use deno_graph::source::ResolveError;
use deno_graph::source::Resolver;
use deno_graph::source::UnknownBuiltInNodeModuleError;
use deno_graph::source::DEFAULT_JSX_IMPORT_SOURCE_MODULE;
use deno_runtime::deno_fs;
use deno_runtime::deno_fs::FileSystem;
use deno_runtime::deno_node::is_builtin_node_module;
use deno_runtime::deno_node::parse_npm_pkg_name;
use deno_runtime::deno_node::NodePermissions;
use deno_runtime::deno_node::NodeResolution;
use deno_runtime::deno_node::NodeResolutionMode;
use deno_runtime::deno_node::NodeResolver;
use deno_runtime::deno_node::NpmResolver as DenoNodeNpmResolver;
use deno_runtime::deno_node::PackageJson;
use deno_runtime::permissions::PermissionsContainer;
use deno_semver::npm::NpmPackageReqReference;
use deno_semver::package::PackageReq;
use import_map::ImportMap;
use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use crate::deno_cli::args::package_json::PackageJsonDeps;
use crate::deno_cli::args::JsxImportSourceConfig;
use crate::deno_cli::args::PackageJsonDepsProvider;
use crate::deno_cli::args::DENO_DISABLE_PEDANTIC_NODE_WARNINGS;
use crate::deno_cli::colors;
use crate::deno_cli::node::CliNodeCodeTranslator;
use crate::deno_cli::npm::ByonmCliNpmResolver;
use crate::deno_cli::npm::CliNpmResolver;
use crate::deno_cli::npm::InnerCliNpmResolverRef;
use crate::deno_cli::util::path::specifier_to_file_path;
use crate::deno_cli::util::sync::AtomicFlag;

pub fn format_range_with_colors(range: &deno_graph::Range) -> String {
  format!(
    "{}:{}:{}",
    colors::cyan(range.specifier.as_str()),
    colors::yellow(&(range.start.line + 1).to_string()),
    colors::yellow(&(range.start.character + 1).to_string())
  )
}

pub struct ModuleCodeStringSource {
  pub code: ModuleCodeString,
  pub found_url: ModuleSpecifier,
  pub media_type: MediaType,
}

#[derive(Debug)]
pub struct CliNodeResolver {
  // not used in the LSP
  cjs_resolutions: Option<Arc<CjsResolutionStore>>,
  fs: Arc<dyn deno_fs::FileSystem>,
  node_resolver: Arc<NodeResolver>,
  // todo(dsherret): remove this pub(crate)
  pub(crate) npm_resolver: Arc<dyn CliNpmResolver>,
}

impl CliNodeResolver {
  pub fn new(
    cjs_resolutions: Option<Arc<CjsResolutionStore>>,
    fs: Arc<dyn deno_fs::FileSystem>,
    node_resolver: Arc<NodeResolver>,
    npm_resolver: Arc<dyn CliNpmResolver>,
  ) -> Self {
    Self {
      cjs_resolutions,
      fs,
      node_resolver,
      npm_resolver,
    }
  }

  pub fn in_npm_package(
    &self,
    referrer: &ModuleSpecifier,
  ) -> bool {
    self.npm_resolver.in_npm_package(referrer)
  }

  pub fn get_closest_package_json(
    &self,
    referrer: &ModuleSpecifier,
    permissions: &dyn NodePermissions,
  ) -> Result<Option<PackageJson>, AnyError> {
    self
      .node_resolver
      .get_closest_package_json(referrer, permissions)
  }

  pub fn resolve_if_in_npm_package(
    &self,
    specifier: &str,
    referrer: &ModuleSpecifier,
    mode: NodeResolutionMode,
    permissions: &PermissionsContainer,
  ) -> Option<Result<Option<NodeResolution>, AnyError>> {
    if self.in_npm_package(referrer) {
      // we're in an npm package, so use node resolution
      Some(self.resolve(specifier, referrer, mode, permissions))
    } else {
      None
    }
  }

  pub fn resolve(
    &self,
    specifier: &str,
    referrer: &ModuleSpecifier,
    mode: NodeResolutionMode,
    permissions: &PermissionsContainer,
  ) -> Result<Option<NodeResolution>, AnyError> {
    self.handle_node_resolve_result(self.node_resolver.resolve(
      specifier,
      referrer,
      mode,
      permissions,
    ))
  }

  pub fn resolve_req_reference(
    &self,
    req_ref: &NpmPackageReqReference,
    permissions: &PermissionsContainer,
    referrer: &ModuleSpecifier,
    mode: NodeResolutionMode,
  ) -> Result<NodeResolution, AnyError> {
    let package_folder = self
      .npm_resolver
      .resolve_pkg_folder_from_deno_module_req(req_ref.req(), referrer)?;
    let maybe_resolution = self.resolve_package_sub_path_from_deno_module(
      &package_folder,
      req_ref.sub_path(),
      referrer,
      mode,
      permissions,
    )?;
    match maybe_resolution {
      Some(resolution) => Ok(resolution),
      None => {
        if self.npm_resolver.as_byonm().is_some() {
          let package_json_path = package_folder.join("package.json");
          if !self.fs.exists_sync(&package_json_path) {
            return Err(anyhow!(
              "Could not find '{}'. Deno expects the node_modules/ directory to be up to date. Did you forget to run `npm install`?",
              package_json_path.display()
            ));
          }
        }
        Err(anyhow!(
          "Failed resolving package subpath for '{}' in '{}'.",
          req_ref,
          package_folder.display()
        ))
      }
    }
  }

  pub fn resolve_package_sub_path_from_deno_module(
    &self,
    package_folder: &Path,
    sub_path: Option<&str>,
    referrer: &ModuleSpecifier,
    mode: NodeResolutionMode,
    permissions: &PermissionsContainer,
  ) -> Result<Option<NodeResolution>, AnyError> {
    self.handle_node_resolve_result(self.node_resolver.resolve_package_subpath_from_deno_module(
      package_folder,
      sub_path,
      referrer,
      mode,
      permissions,
    ))
  }

  pub fn handle_if_in_node_modules(
    &self,
    specifier: ModuleSpecifier,
  ) -> Result<ModuleSpecifier, AnyError> {
    // skip canonicalizing if we definitely know it's unnecessary
    if specifier.scheme() == "file" && specifier.path().contains("/node_modules/") {
      // Specifiers in the node_modules directory are canonicalized
      // so canoncalize then check if it's in the node_modules directory.
      // If so, check if we need to store this specifier as being a CJS
      // resolution.
      let specifier = crate::deno_cli::node::resolve_specifier_into_node_modules(&specifier);
      if self.in_npm_package(&specifier) {
        if let Some(cjs_resolutions) = &self.cjs_resolutions {
          let resolution = self.node_resolver.url_to_node_resolution(specifier)?;
          if let NodeResolution::CommonJs(specifier) = &resolution {
            cjs_resolutions.insert(specifier.clone());
          }
          return Ok(resolution.into_url());
        } else {
          return Ok(specifier);
        }
      }
    }

    Ok(specifier)
  }

  fn handle_node_resolve_result(
    &self,
    result: Result<Option<NodeResolution>, AnyError>,
  ) -> Result<Option<NodeResolution>, AnyError> {
    match result? {
      Some(response) => {
        if let NodeResolution::CommonJs(specifier) = &response {
          // remember that this was a common js resolution
          if let Some(cjs_resolutions) = &self.cjs_resolutions {
            cjs_resolutions.insert(specifier.clone());
          }
        }
        Ok(Some(response))
      }
      None => Ok(None),
    }
  }
}

pub struct NpmModuleLoader {
  cjs_resolutions: Arc<CjsResolutionStore>,
  node_code_translator: Arc<CliNodeCodeTranslator>,
  fs: Arc<dyn deno_fs::FileSystem>,
  node_resolver: Arc<CliNodeResolver>,
}

impl NpmModuleLoader {
  pub fn new(
    cjs_resolutions: Arc<CjsResolutionStore>,
    node_code_translator: Arc<CliNodeCodeTranslator>,
    fs: Arc<dyn deno_fs::FileSystem>,
    node_resolver: Arc<CliNodeResolver>,
  ) -> Self {
    Self {
      cjs_resolutions,
      node_code_translator,
      fs,
      node_resolver,
    }
  }

  pub fn maybe_prepare_load(
    &self,
    specifier: &ModuleSpecifier,
  ) -> Option<Result<(), AnyError>> {
    if self.node_resolver.in_npm_package(specifier) {
      // nothing to prepare
      Some(Ok(()))
    } else {
      None
    }
  }

  pub fn load_sync_if_in_npm_package(
    &self,
    specifier: &ModuleSpecifier,
    maybe_referrer: Option<&ModuleSpecifier>,
    permissions: &PermissionsContainer,
  ) -> Option<Result<ModuleCodeStringSource, AnyError>> {
    if self.node_resolver.in_npm_package(specifier) {
      Some(self.load_sync(specifier, maybe_referrer, permissions))
    } else {
      None
    }
  }

  fn load_sync(
    &self,
    specifier: &ModuleSpecifier,
    maybe_referrer: Option<&ModuleSpecifier>,
    permissions: &PermissionsContainer,
  ) -> Result<ModuleCodeStringSource, AnyError> {
    let file_path = specifier.to_file_path().unwrap();
    let code = self
      .fs
      .read_text_file_sync(&file_path)
      .map_err(AnyError::from)
      .with_context(|| {
        if file_path.is_dir() {
          // directory imports are not allowed when importing from an
          // ES module, so provide the user with a helpful error message
          let dir_path = file_path;
          let mut msg = "Directory import ".to_string();
          msg.push_str(&dir_path.to_string_lossy());
          if let Some(referrer) = &maybe_referrer {
            msg.push_str(" is not supported resolving import from ");
            msg.push_str(referrer.as_str());
            let entrypoint_name = ["index.mjs", "index.js", "index.cjs"]
              .iter()
              .find(|e| dir_path.join(e).is_file());
            if let Some(entrypoint_name) = entrypoint_name {
              msg.push_str("\nDid you mean to import ");
              msg.push_str(entrypoint_name);
              msg.push_str(" within the directory?");
            }
          }
          msg
        } else {
          let mut msg = "Unable to load ".to_string();
          msg.push_str(&file_path.to_string_lossy());
          if let Some(referrer) = &maybe_referrer {
            msg.push_str(" imported from ");
            msg.push_str(referrer.as_str());
          }
          msg
        }
      })?;

    let code = if self.cjs_resolutions.contains(specifier) {
      // translate cjs to esm if it's cjs and inject node globals
      self
        .node_code_translator
        .translate_cjs_to_esm(specifier, Some(code.as_str()), permissions)?
    } else {
      // esm and json code is untouched
      code
    };
    Ok(ModuleCodeStringSource {
      code: code.into(),
      found_url: specifier.clone(),
      media_type: MediaType::from_specifier(specifier),
    })
  }
}

/// Keeps track of what module specifiers were resolved as CJS.
#[derive(Debug, Default)]
pub struct CjsResolutionStore(Mutex<HashSet<ModuleSpecifier>>);

impl CjsResolutionStore {
  pub fn contains(
    &self,
    specifier: &ModuleSpecifier,
  ) -> bool {
    self.0.lock().contains(specifier)
  }

  pub fn insert(
    &self,
    specifier: ModuleSpecifier,
  ) {
    self.0.lock().insert(specifier);
  }
}

/// Result of checking if a specifier is mapped via
/// an import map or package.json.
pub enum MappedResolution {
  None,
  PackageJson(ModuleSpecifier),
  ImportMap(ModuleSpecifier),
}

impl MappedResolution {
  pub fn into_specifier(self) -> Option<ModuleSpecifier> {
    match self {
      MappedResolution::None => Option::None,
      MappedResolution::PackageJson(specifier) => Some(specifier),
      MappedResolution::ImportMap(specifier) => Some(specifier),
    }
  }
}

/// Resolver for specifiers that could be mapped via an
/// import map or package.json.
#[derive(Debug)]
pub struct MappedSpecifierResolver {
  maybe_import_map: Option<Arc<ImportMap>>,
  package_json_deps_provider: Arc<PackageJsonDepsProvider>,
}

impl MappedSpecifierResolver {
  pub fn new(
    maybe_import_map: Option<Arc<ImportMap>>,
    package_json_deps_provider: Arc<PackageJsonDepsProvider>,
  ) -> Self {
    Self {
      maybe_import_map,
      package_json_deps_provider,
    }
  }

  pub fn resolve(
    &self,
    specifier: &str,
    referrer: &ModuleSpecifier,
  ) -> Result<MappedResolution, AnyError> {
    // attempt to resolve with the import map first
    let maybe_import_map_err = match self
      .maybe_import_map
      .as_ref()
      .map(|import_map| import_map.resolve(specifier, referrer))
    {
      Some(Ok(value)) => return Ok(MappedResolution::ImportMap(value)),
      Some(Err(err)) => Some(err),
      None => None,
    };

    // then with package.json
    if let Some(deps) = self.package_json_deps_provider.deps() {
      if let Some(specifier) = resolve_package_json_dep(specifier, deps)? {
        return Ok(MappedResolution::PackageJson(specifier));
      }
    }

    // otherwise, surface the import map error or try resolving when has no import map
    if let Some(err) = maybe_import_map_err {
      Err(err.into())
    } else {
      Ok(MappedResolution::None)
    }
  }
}

/// A resolver that takes care of resolution, taking into account loaded
/// import map, JSX settings.
#[derive(Debug)]
pub struct CliGraphResolver {
  sloppy_imports_resolver: Option<SloppyImportsResolver>,
  mapped_specifier_resolver: MappedSpecifierResolver,
  maybe_default_jsx_import_source: Option<String>,
  maybe_jsx_import_source_module: Option<String>,
  maybe_vendor_specifier: Option<ModuleSpecifier>,
  node_resolver: Option<Arc<CliNodeResolver>>,
  npm_resolver: Option<Arc<dyn CliNpmResolver>>,
  found_package_json_dep_flag: Arc<AtomicFlag>,
  bare_node_builtins_enabled: bool,
}

pub struct CliGraphResolverOptions<'a> {
  pub sloppy_imports_resolver: Option<SloppyImportsResolver>,
  pub node_resolver: Option<Arc<CliNodeResolver>>,
  pub npm_resolver: Option<Arc<dyn CliNpmResolver>>,
  pub package_json_deps_provider: Arc<PackageJsonDepsProvider>,
  pub maybe_jsx_import_source_config: Option<JsxImportSourceConfig>,
  pub maybe_import_map: Option<Arc<ImportMap>>,
  pub maybe_vendor_dir: Option<&'a PathBuf>,
  pub bare_node_builtins_enabled: bool,
}

impl CliGraphResolver {
  pub fn new(options: CliGraphResolverOptions) -> Self {
    let is_byonm = options
      .npm_resolver
      .as_ref()
      .map(|n| n.as_byonm().is_some())
      .unwrap_or(false);
    Self {
      sloppy_imports_resolver: options.sloppy_imports_resolver,
      mapped_specifier_resolver: MappedSpecifierResolver::new(
        options.maybe_import_map,
        if is_byonm {
          // don't resolve from the root package.json deps for byonm
          Arc::new(PackageJsonDepsProvider::new(None))
        } else {
          options.package_json_deps_provider
        },
      ),
      maybe_default_jsx_import_source: options
        .maybe_jsx_import_source_config
        .as_ref()
        .and_then(|c| c.default_specifier.clone()),
      maybe_jsx_import_source_module: options.maybe_jsx_import_source_config.map(|c| c.module),
      maybe_vendor_specifier: options
        .maybe_vendor_dir
        .and_then(|v| ModuleSpecifier::from_directory_path(v).ok()),
      node_resolver: options.node_resolver,
      npm_resolver: options.npm_resolver,
      found_package_json_dep_flag: Default::default(),
      bare_node_builtins_enabled: options.bare_node_builtins_enabled,
    }
  }

  pub fn as_graph_resolver(&self) -> &dyn Resolver {
    self
  }

  pub fn as_graph_npm_resolver(&self) -> &dyn NpmResolver {
    self
  }

  pub fn found_package_json_dep(&self) -> bool {
    self.found_package_json_dep_flag.is_raised()
  }

  fn check_surface_byonm_node_error(
    &self,
    specifier: &str,
    referrer: &ModuleSpecifier,
    mode: NodeResolutionMode,
    original_err: AnyError,
    resolver: &ByonmCliNpmResolver,
  ) -> Result<(), AnyError> {
    if let Ok((pkg_name, _, _)) = parse_npm_pkg_name(specifier, referrer) {
      match resolver.resolve_package_folder_from_package(&pkg_name, referrer, mode) {
        Ok(_) => {
          return Err(original_err);
        }
        Err(_) => {
          if resolver
            .find_ancestor_package_json_with_dep(&pkg_name, referrer)
            .is_some()
          {
            return Err(anyhow!(
              concat!(
                "Could not resolve \"{}\", but found it in a package.json. ",
                "Deno expects the node_modules/ directory to be up to date. ",
                "Did you forget to run `npm install`?"
              ),
              specifier
            ));
          }
        }
      }
    }
    Ok(())
  }
}

impl Resolver for CliGraphResolver {
  fn default_jsx_import_source(&self) -> Option<String> {
    self.maybe_default_jsx_import_source.clone()
  }

  fn jsx_import_source_module(&self) -> &str {
    self
      .maybe_jsx_import_source_module
      .as_deref()
      .unwrap_or(DEFAULT_JSX_IMPORT_SOURCE_MODULE)
  }

  fn resolve(
    &self,
    specifier: &str,
    referrer_range: &deno_graph::Range,
    mode: ResolutionMode,
  ) -> Result<ModuleSpecifier, ResolveError> {
    fn to_node_mode(mode: ResolutionMode) -> NodeResolutionMode {
      match mode {
        ResolutionMode::Execution => NodeResolutionMode::Execution,
        ResolutionMode::Types => NodeResolutionMode::Types,
      }
    }

    let referrer = &referrer_range.specifier;
    let result: Result<_, ResolveError> = self
      .mapped_specifier_resolver
      .resolve(specifier, referrer)
      .map_err(|err| err.into())
      .and_then(|resolution| match resolution {
        MappedResolution::ImportMap(specifier) => Ok(specifier),
        MappedResolution::PackageJson(specifier) => {
          // found a specifier in the package.json, so mark that
          // we need to do an "npm install" later
          self.found_package_json_dep_flag.raise();
          Ok(specifier)
        }
        MappedResolution::None => {
          deno_graph::resolve_import(specifier, &referrer_range.specifier).map_err(|err| err.into())
        }
      });

    // do sloppy imports resolution if enabled
    let result = if let Some(sloppy_imports_resolver) = &self.sloppy_imports_resolver {
      result.map(|specifier| {
        sloppy_imports_resolve(sloppy_imports_resolver, specifier, referrer_range, mode)
      })
    } else {
      result
    };

    // When the user is vendoring, don't allow them to import directly from the vendor/ directory
    // as it might cause them confusion or duplicate dependencies. Additionally, this folder has
    // special treatment in the language server so it will definitely cause issues/confusion there
    // if they do this.
    if let Some(vendor_specifier) = &self.maybe_vendor_specifier {
      if let Ok(specifier) = &result {
        if specifier.as_str().starts_with(vendor_specifier.as_str()) {
          return Err(ResolveError::Other(anyhow!("Importing from the vendor directory is not permitted. Use a remote specifier instead or disable vendoring.")));
        }
      }
    }

    if let Some(resolver) = self.npm_resolver.as_ref().and_then(|r| r.as_byonm()) {
      match &result {
        Ok(specifier) => {
          if let Ok(npm_req_ref) = NpmPackageReqReference::from_specifier(specifier) {
            let node_resolver = self.node_resolver.as_ref().unwrap();
            return node_resolver
              .resolve_req_reference(
                &npm_req_ref,
                &PermissionsContainer::allow_all(),
                referrer,
                to_node_mode(mode),
              )
              .map(|res| res.into_url())
              .map_err(|err| err.into());
          }
        }
        Err(_) => {
          if referrer.scheme() == "file" {
            if let Some(node_resolver) = &self.node_resolver {
              let node_result = node_resolver.resolve(
                specifier,
                referrer,
                to_node_mode(mode),
                &PermissionsContainer::allow_all(),
              );
              match node_result {
                Ok(Some(res)) => {
                  return Ok(res.into_url());
                }
                Ok(None) => {
                  self
                    .check_surface_byonm_node_error(
                      specifier,
                      referrer,
                      to_node_mode(mode),
                      anyhow!("Cannot find \"{}\"", specifier),
                      resolver,
                    )
                    .map_err(ResolveError::Other)?;
                }
                Err(err) => {
                  self
                    .check_surface_byonm_node_error(
                      specifier,
                      referrer,
                      to_node_mode(mode),
                      err,
                      resolver,
                    )
                    .map_err(ResolveError::Other)?;
                }
              }
            }
          }
        }
      }
    }

    let specifier = result?;
    match &self.node_resolver {
      Some(node_resolver) => node_resolver
        .handle_if_in_node_modules(specifier)
        .map_err(|e| e.into()),
      None => Ok(specifier),
    }
  }
}

fn sloppy_imports_resolve(
  resolver: &SloppyImportsResolver,
  specifier: ModuleSpecifier,
  referrer_range: &deno_graph::Range,
  mode: ResolutionMode,
) -> ModuleSpecifier {
  let resolution = resolver.resolve(&specifier, mode);
  if mode.is_types() {
    // don't bother warning for types resolution because
    // we already probably warned during execution resolution
    match resolution {
      SloppyImportsResolution::None(_) => return specifier, // avoid a clone
      _ => return resolution.into_specifier().into_owned(),
    }
  }

  let hint_message = match &resolution {
    SloppyImportsResolution::JsToTs(to_specifier) => {
      let to_media_type = MediaType::from_specifier(to_specifier);
      let from_media_type = MediaType::from_specifier(&specifier);
      format!(
        "update {} extension to {}",
        from_media_type.as_ts_extension(),
        to_media_type.as_ts_extension()
      )
    }
    SloppyImportsResolution::NoExtension(to_specifier) => {
      let to_media_type = MediaType::from_specifier(to_specifier);
      format!("add {} extension", to_media_type.as_ts_extension())
    }
    SloppyImportsResolution::Directory(to_specifier) => {
      let file_name = to_specifier
        .path()
        .rsplit_once('/')
        .map(|(_, file_name)| file_name)
        .unwrap_or(to_specifier.path());
      format!("specify path to {} file in directory instead", file_name)
    }
    SloppyImportsResolution::None(_) => return specifier,
  };
  // show a warning when this happens in order to drive
  // the user towards correcting these specifiers
  if !*DENO_DISABLE_PEDANTIC_NODE_WARNINGS {
    log::warn!(
      "{} Sloppy module resolution {}\n    at {}",
      crate::deno_cli::colors::yellow("Warning"),
      crate::deno_cli::colors::gray(format!("(hint: {})", hint_message)).to_string(),
      if referrer_range.end == deno_graph::Position::zeroed() {
        // not worth showing the range in this case
        crate::deno_cli::colors::cyan(referrer_range.specifier.as_str()).to_string()
      } else {
        format_range_with_colors(referrer_range)
      },
    );
  }

  resolution.into_specifier().into_owned()
}

fn resolve_package_json_dep(
  specifier: &str,
  deps: &PackageJsonDeps,
) -> Result<Option<ModuleSpecifier>, AnyError> {
  for (bare_specifier, req_result) in deps {
    if specifier.starts_with(bare_specifier) {
      let path = &specifier[bare_specifier.len()..];
      if path.is_empty() || path.starts_with('/') {
        let req = req_result.as_ref().map_err(|err| {
          anyhow!(
            "Parsing version constraints in the application-level package.json is more strict at the moment.\n\n{:#}",
            err.clone()
          )
        })?;
        return Ok(Some(ModuleSpecifier::parse(&format!("npm:{req}{path}"))?));
      }
    }
  }

  Ok(None)
}

impl NpmResolver for CliGraphResolver {
  fn resolve_builtin_node_module(
    &self,
    specifier: &ModuleSpecifier,
  ) -> Result<Option<String>, UnknownBuiltInNodeModuleError> {
    if specifier.scheme() != "node" {
      return Ok(None);
    }

    let module_name = specifier.path().to_string();
    if is_builtin_node_module(&module_name) {
      Ok(Some(module_name))
    } else {
      Err(UnknownBuiltInNodeModuleError { module_name })
    }
  }

  fn on_resolve_bare_builtin_node_module(
    &self,
    module_name: &str,
    range: &deno_graph::Range,
  ) {
    let deno_graph::Range {
      start, specifier, ..
    } = range;
    let line = start.line + 1;
    let column = start.character + 1;
    if !*DENO_DISABLE_PEDANTIC_NODE_WARNINGS {
      log::warn!("Warning: Resolving \"{module_name}\" as \"node:{module_name}\" at {specifier}:{line}:{column}. If you want to use a built-in Node module, add a \"node:\" prefix.")
    }
  }

  fn load_and_cache_npm_package_info(
    &self,
    package_name: &str,
  ) -> LocalBoxFuture<'static, Result<(), AnyError>> {
    match &self.npm_resolver {
      Some(npm_resolver) if npm_resolver.as_managed().is_some() => {
        let package_name = package_name.to_string();
        let npm_resolver = npm_resolver.clone();
        async move {
          if let Some(managed) = npm_resolver.as_managed() {
            managed.cache_package_info(&package_name).await?;
          }
          Ok(())
        }
        .boxed()
      }
      _ => {
        // return it succeeded and error at the import site below
        Box::pin(future::ready(Ok(())))
      }
    }
  }

  fn resolve_npm(
    &self,
    package_req: &PackageReq,
  ) -> NpmPackageReqResolution {
    match &self.npm_resolver {
      Some(npm_resolver) => match npm_resolver.as_inner() {
        InnerCliNpmResolverRef::Managed(npm_resolver) => {
          npm_resolver.resolve_npm_for_deno_graph(package_req)
        }
        // if we are using byonm, then this should never be called because
        // we don't use deno_graph's npm resolution in this case
        InnerCliNpmResolverRef::Byonm(_) => unreachable!(),
      },
      None => NpmPackageReqResolution::Err(anyhow!(
        "npm specifiers were requested; but --no-npm is specified"
      )),
    }
  }

  fn enables_bare_builtin_node_module(&self) -> bool {
    self.bare_node_builtins_enabled
  }
}

#[derive(Debug)]
struct SloppyImportsStatCache {
  fs: Arc<dyn FileSystem>,
  cache: Mutex<HashMap<PathBuf, Option<SloppyImportsFsEntry>>>,
}

impl SloppyImportsStatCache {
  pub fn new(fs: Arc<dyn FileSystem>) -> Self {
    Self {
      fs,
      cache: Default::default(),
    }
  }

  pub fn stat_sync(
    &self,
    path: &Path,
  ) -> Option<SloppyImportsFsEntry> {
    // there will only ever be one thread in here at a
    // time, so it's ok to hold the lock for so long
    let mut cache = self.cache.lock();
    if let Some(entry) = cache.get(path) {
      return *entry;
    }

    let entry = self
      .fs
      .stat_sync(path)
      .ok()
      .and_then(|stat| SloppyImportsFsEntry::from_fs_stat(&stat));
    cache.insert(path.to_owned(), entry);
    entry
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SloppyImportsFsEntry {
  File,
  Dir,
}

impl SloppyImportsFsEntry {
  pub fn from_fs_stat(stat: &deno_runtime::deno_io::fs::FsStat) -> Option<SloppyImportsFsEntry> {
    if stat.is_file {
      Some(SloppyImportsFsEntry::File)
    } else if stat.is_directory {
      Some(SloppyImportsFsEntry::Dir)
    } else {
      None
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SloppyImportsResolution<'a> {
  /// No sloppy resolution was found.
  None(&'a ModuleSpecifier),
  /// Ex. `./file.js` to `./file.ts`
  JsToTs(ModuleSpecifier),
  /// Ex. `./file` to `./file.ts`
  NoExtension(ModuleSpecifier),
  /// Ex. `./dir` to `./dir/index.ts`
  Directory(ModuleSpecifier),
}

impl<'a> SloppyImportsResolution<'a> {
  pub fn as_specifier(&self) -> &ModuleSpecifier {
    match self {
      Self::None(specifier) => specifier,
      Self::JsToTs(specifier) => specifier,
      Self::NoExtension(specifier) => specifier,
      Self::Directory(specifier) => specifier,
    }
  }

  pub fn into_specifier(self) -> Cow<'a, ModuleSpecifier> {
    match self {
      Self::None(specifier) => Cow::Borrowed(specifier),
      Self::JsToTs(specifier) => Cow::Owned(specifier),
      Self::NoExtension(specifier) => Cow::Owned(specifier),
      Self::Directory(specifier) => Cow::Owned(specifier),
    }
  }

  pub fn as_suggestion_message(&self) -> Option<String> {
    Some(format!("Maybe {}", self.as_base_message()?))
  }

  pub fn as_lsp_quick_fix_message(&self) -> Option<String> {
    let message = self.as_base_message()?;
    let mut chars = message.chars();
    Some(format!(
      "{}{}.",
      chars.next().unwrap().to_uppercase(),
      chars.as_str()
    ))
  }

  fn as_base_message(&self) -> Option<String> {
    match self {
      SloppyImportsResolution::None(_) => None,
      SloppyImportsResolution::JsToTs(specifier) => {
        let media_type = MediaType::from_specifier(specifier);
        Some(format!(
          "change the extension to '{}'",
          media_type.as_ts_extension()
        ))
      }
      SloppyImportsResolution::NoExtension(specifier) => {
        let media_type = MediaType::from_specifier(specifier);
        Some(format!(
          "add a '{}' extension",
          media_type.as_ts_extension()
        ))
      }
      SloppyImportsResolution::Directory(specifier) => {
        let file_name = specifier
          .path()
          .rsplit_once('/')
          .map(|(_, file_name)| file_name)
          .unwrap_or(specifier.path());
        Some(format!(
          "specify path to '{}' file in directory instead",
          file_name
        ))
      }
    }
  }
}

#[derive(Debug)]
pub struct SloppyImportsResolver {
  stat_cache: SloppyImportsStatCache,
}

impl SloppyImportsResolver {
  pub fn new(fs: Arc<dyn FileSystem>) -> Self {
    Self {
      stat_cache: SloppyImportsStatCache::new(fs),
    }
  }

  pub fn resolve_with_fs<'a>(
    fs: &dyn FileSystem,
    specifier: &'a ModuleSpecifier,
    mode: ResolutionMode,
  ) -> SloppyImportsResolution<'a> {
    Self::resolve_with_stat_sync(specifier, mode, |path| {
      fs.stat_sync(path)
        .ok()
        .and_then(|stat| SloppyImportsFsEntry::from_fs_stat(&stat))
    })
  }

  pub fn resolve_with_stat_sync(
    specifier: &ModuleSpecifier,
    mode: ResolutionMode,
    stat_sync: impl Fn(&Path) -> Option<SloppyImportsFsEntry>,
  ) -> SloppyImportsResolution {
    fn path_without_ext(
      path: &Path,
      media_type: MediaType,
    ) -> Option<Cow<str>> {
      let old_path_str = path.to_string_lossy();
      match media_type {
        MediaType::Unknown => Some(old_path_str),
        _ => old_path_str
          .strip_suffix(media_type.as_ts_extension())
          .map(|s| Cow::Owned(s.to_string())),
      }
    }

    fn media_types_to_paths(
      path_no_ext: &str,
      probe_media_type_types: Vec<MediaType>,
    ) -> Vec<PathBuf> {
      probe_media_type_types
        .into_iter()
        .map(|media_type| PathBuf::from(format!("{}{}", path_no_ext, media_type.as_ts_extension())))
        .collect::<Vec<_>>()
    }

    if specifier.scheme() != "file" {
      return SloppyImportsResolution::None(specifier);
    }

    let Ok(path) = specifier_to_file_path(specifier) else {
      return SloppyImportsResolution::None(specifier);
    };
    let mut is_dir_resolution = false;
    let mut is_no_ext_resolution = false;
    let probe_paths = match (stat_sync)(&path) {
      Some(SloppyImportsFsEntry::File) => {
        if mode.is_types() {
          let media_type = MediaType::from_specifier(specifier);
          // attempt to resolve the .d.ts file before the .js file
          let probe_media_type_types = match media_type {
            MediaType::JavaScript => {
              vec![MediaType::Dts, MediaType::JavaScript]
            }
            MediaType::Mjs => {
              vec![MediaType::Dmts, MediaType::Dts, MediaType::Mjs]
            }
            MediaType::Cjs => {
              vec![MediaType::Dcts, MediaType::Dts, MediaType::Cjs]
            }
            _ => return SloppyImportsResolution::None(specifier),
          };
          let Some(path_no_ext) = path_without_ext(&path, media_type) else {
            return SloppyImportsResolution::None(specifier);
          };
          media_types_to_paths(&path_no_ext, probe_media_type_types)
        } else {
          return SloppyImportsResolution::None(specifier);
        }
      }
      Some(SloppyImportsFsEntry::Dir) => {
        is_dir_resolution = true;
        // try to resolve at the index file
        if mode.is_types() {
          vec![
            path.join("index.ts"),
            path.join("index.mts"),
            path.join("index.d.ts"),
            path.join("index.d.mts"),
            path.join("index.js"),
            path.join("index.mjs"),
            path.join("index.tsx"),
            path.join("index.jsx"),
          ]
        } else {
          vec![
            path.join("index.ts"),
            path.join("index.mts"),
            path.join("index.tsx"),
            path.join("index.js"),
            path.join("index.mjs"),
            path.join("index.jsx"),
          ]
        }
      }
      None => {
        let media_type = MediaType::from_specifier(specifier);
        let probe_media_type_types = match media_type {
          MediaType::JavaScript => {
            if mode.is_types() {
              vec![MediaType::TypeScript, MediaType::Tsx, MediaType::Dts]
            } else {
              vec![MediaType::TypeScript, MediaType::Tsx]
            }
          }
          MediaType::Jsx => vec![MediaType::Tsx],
          MediaType::Mjs => {
            if mode.is_types() {
              vec![MediaType::Mts, MediaType::Dmts, MediaType::Dts]
            } else {
              vec![MediaType::Mts]
            }
          }
          MediaType::Cjs => {
            if mode.is_types() {
              vec![MediaType::Cts, MediaType::Dcts, MediaType::Dts]
            } else {
              vec![MediaType::Cts]
            }
          }
          MediaType::TypeScript
          | MediaType::Mts
          | MediaType::Cts
          | MediaType::Dts
          | MediaType::Dmts
          | MediaType::Dcts
          | MediaType::Tsx
          | MediaType::Json
          | MediaType::Wasm
          | MediaType::TsBuildInfo
          | MediaType::SourceMap => return SloppyImportsResolution::None(specifier),
          MediaType::Unknown => {
            is_no_ext_resolution = true;
            if mode.is_types() {
              vec![
                MediaType::TypeScript,
                MediaType::Tsx,
                MediaType::Mts,
                MediaType::Dts,
                MediaType::Dmts,
                MediaType::Dcts,
                MediaType::JavaScript,
                MediaType::Jsx,
                MediaType::Mjs,
              ]
            } else {
              vec![
                MediaType::TypeScript,
                MediaType::JavaScript,
                MediaType::Tsx,
                MediaType::Jsx,
                MediaType::Mts,
                MediaType::Mjs,
              ]
            }
          }
        };
        let Some(path_no_ext) = path_without_ext(&path, media_type) else {
          return SloppyImportsResolution::None(specifier);
        };
        media_types_to_paths(&path_no_ext, probe_media_type_types)
      }
    };

    for probe_path in probe_paths {
      if (stat_sync)(&probe_path) == Some(SloppyImportsFsEntry::File) {
        if let Ok(specifier) = ModuleSpecifier::from_file_path(probe_path) {
          if is_dir_resolution {
            return SloppyImportsResolution::Directory(specifier);
          } else if is_no_ext_resolution {
            return SloppyImportsResolution::NoExtension(specifier);
          } else {
            return SloppyImportsResolution::JsToTs(specifier);
          }
        }
      }
    }

    SloppyImportsResolution::None(specifier)
  }

  pub fn resolve<'a>(
    &self,
    specifier: &'a ModuleSpecifier,
    mode: ResolutionMode,
  ) -> SloppyImportsResolution<'a> {
    Self::resolve_with_stat_sync(specifier, mode, |path| self.stat_cache.stat_sync(path))
  }
}
