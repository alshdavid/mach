// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use std::fmt::Write as _;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use deno_ast::ModuleSpecifier;
use deno_core::anyhow::bail;
use deno_core::anyhow::Context;
use deno_core::error::AnyError;
use deno_core::futures::future::LocalBoxFuture;
use deno_core::parking_lot::Mutex;
use deno_graph::source::ResolutionMode;
use deno_graph::JsModule;
use deno_graph::Module;
use deno_graph::ModuleGraph;
use deno_runtime::deno_fs;
use import_map::ImportMap;
use import_map::SpecifierMap;

use crate::deno_cli::args::JsxImportSourceConfig;
use crate::deno_cli::args::Lockfile;
use crate::deno_cli::cache::ParsedSourceCache;
use crate::deno_cli::graph_util;
use crate::deno_cli::graph_util::graph_lock_or_exit;
use crate::deno_cli::tools::vendor::import_map::BuildImportMapInput;

use super::analyze::has_default_export;
use super::import_map::build_import_map;
use super::mappings::Mappings;
use super::mappings::ProxiedModule;
use super::specifiers::is_remote_specifier;

/// Allows substituting the environment for testing purposes.
pub trait VendorEnvironment {
  fn cwd(&self) -> Result<PathBuf, AnyError>;
  fn create_dir_all(&self, dir_path: &Path) -> Result<(), AnyError>;
  fn write_file(&self, file_path: &Path, bytes: &[u8]) -> Result<(), AnyError>;
  fn path_exists(&self, path: &Path) -> bool;
}

pub struct RealVendorEnvironment;

impl VendorEnvironment for RealVendorEnvironment {
  fn cwd(&self) -> Result<PathBuf, AnyError> {
    Ok(std::env::current_dir()?)
  }

  fn create_dir_all(&self, dir_path: &Path) -> Result<(), AnyError> {
    Ok(std::fs::create_dir_all(dir_path)?)
  }

  fn write_file(&self, file_path: &Path, bytes: &[u8]) -> Result<(), AnyError> {
    std::fs::write(file_path, bytes)
      .with_context(|| format!("Failed writing {}", file_path.display()))
  }

  fn path_exists(&self, path: &Path) -> bool {
    path.exists()
  }
}

type BuildGraphFuture = LocalBoxFuture<'static, Result<ModuleGraph, AnyError>>;

pub struct BuildInput<
  'a,
  TBuildGraphFn: FnOnce(Vec<ModuleSpecifier>) -> BuildGraphFuture,
  TEnvironment: VendorEnvironment,
> {
  pub entry_points: Vec<ModuleSpecifier>,
  pub build_graph: TBuildGraphFn,
  pub parsed_source_cache: &'a ParsedSourceCache,
  pub output_dir: &'a Path,
  pub maybe_original_import_map: Option<&'a ImportMap>,
  pub maybe_lockfile: Option<Arc<Mutex<Lockfile>>>,
  pub maybe_jsx_import_source: Option<&'a JsxImportSourceConfig>,
  pub resolver: &'a dyn deno_graph::source::Resolver,
  pub environment: &'a TEnvironment,
}

pub struct BuildOutput {
  pub vendored_count: usize,
  pub graph: ModuleGraph,
}

/// Vendors remote modules and returns how many were vendored.
pub async fn build<
  TBuildGraphFn: FnOnce(Vec<ModuleSpecifier>) -> BuildGraphFuture,
  TEnvironment: VendorEnvironment,
>(
  input: BuildInput<'_, TBuildGraphFn, TEnvironment>,
) -> Result<BuildOutput, AnyError> {
  let BuildInput {
    mut entry_points,
    build_graph,
    parsed_source_cache,
    output_dir,
    maybe_original_import_map: original_import_map,
    maybe_lockfile,
    maybe_jsx_import_source: jsx_import_source,
    resolver,
    environment,
  } = input;
  assert!(output_dir.is_absolute());
  let output_dir_specifier =
    ModuleSpecifier::from_directory_path(output_dir).unwrap();

  if let Some(original_im) = &original_import_map {
    validate_original_import_map(original_im, &output_dir_specifier)?;
  }

  // add the jsx import source to the entry points to ensure it is always vendored
  if let Some(jsx_import_source) = jsx_import_source {
    if let Some(specifier_text) = jsx_import_source.maybe_specifier_text() {
      if let Ok(specifier) = resolver.resolve(
        &specifier_text,
        &deno_graph::Range {
          specifier: jsx_import_source.base_url.clone(),
          start: deno_graph::Position::zeroed(),
          end: deno_graph::Position::zeroed(),
        },
        ResolutionMode::Execution,
      ) {
        entry_points.push(specifier);
      }
    }
  }

  let graph = build_graph(entry_points).await?;

  // check the lockfile
  if let Some(lockfile) = maybe_lockfile {
    graph_lock_or_exit(&graph, &mut lockfile.lock());
  }

  // surface any errors
  graph_util::graph_valid(
    &graph,
    &deno_fs::RealFs,
    &graph.roots,
    graph_util::GraphValidOptions {
      is_vendoring: true,
      check_js: true,
      follow_type_only: true,
    },
  )?;

  // figure out how to map remote modules to local
  let all_modules = graph.modules().collect::<Vec<_>>();
  let remote_modules = all_modules
    .iter()
    .filter(|m| is_remote_specifier(m.specifier()))
    .copied()
    .collect::<Vec<_>>();
  let mappings =
    Mappings::from_remote_modules(&graph, &remote_modules, output_dir)?;

  // write out all the files
  for module in &remote_modules {
    let source = match module {
      Module::Js(module) => &module.source,
      Module::Json(module) => &module.source,
      Module::Node(_) | Module::Npm(_) | Module::External(_) => continue,
    };
    let specifier = module.specifier();
    let local_path = mappings
      .proxied_path(specifier)
      .unwrap_or_else(|| mappings.local_path(specifier));

    environment.create_dir_all(local_path.parent().unwrap())?;
    environment.write_file(&local_path, source.as_bytes())?;
  }

  // write out the proxies
  for (specifier, proxied_module) in mappings.proxied_modules() {
    let proxy_path = mappings.local_path(specifier);
    let module = graph.get(specifier).unwrap().js().unwrap();
    let text =
      build_proxy_module_source(module, proxied_module, parsed_source_cache)?;

    environment.write_file(&proxy_path, text.as_bytes())?;
  }

  // create the import map if necessary
  if !remote_modules.is_empty() {
    let import_map_path = output_dir.join("import_map.json");
    let import_map_text = build_import_map(BuildImportMapInput {
      base_dir: &output_dir_specifier,
      graph: &graph,
      modules: &all_modules,
      mappings: &mappings,
      original_import_map,
      jsx_import_source,
      resolver,
      parsed_source_cache,
    })?;
    environment.write_file(&import_map_path, import_map_text.as_bytes())?;
  }

  Ok(BuildOutput {
    vendored_count: remote_modules.len(),
    graph,
  })
}

fn validate_original_import_map(
  import_map: &ImportMap,
  output_dir: &ModuleSpecifier,
) -> Result<(), AnyError> {
  fn validate_imports(
    imports: &SpecifierMap,
    output_dir: &ModuleSpecifier,
  ) -> Result<(), AnyError> {
    for entry in imports.entries() {
      if let Some(value) = entry.value {
        if value.as_str().starts_with(output_dir.as_str()) {
          bail!(
            "Providing an existing import map with entries for the output directory is not supported (\"{}\": \"{}\").",
            entry.raw_key,
            entry.raw_value.unwrap_or("<INVALID>"),
          );
        }
      }
    }
    Ok(())
  }

  validate_imports(import_map.imports(), output_dir)?;

  for scope in import_map.scopes() {
    if scope.key.starts_with(output_dir.as_str()) {
      bail!(
        "Providing an existing import map with a scope for the output directory is not supported (\"{}\").",
        scope.raw_key,
      );
    }
    validate_imports(scope.imports, output_dir)?;
  }

  Ok(())
}

fn build_proxy_module_source(
  module: &JsModule,
  proxied_module: &ProxiedModule,
  parsed_source_cache: &ParsedSourceCache,
) -> Result<String, AnyError> {
  let mut text = String::new();
  writeln!(
    text,
    "// @deno-types=\"{}\"",
    proxied_module.declaration_specifier
  )
  .unwrap();

  let relative_specifier = format!(
    "./{}",
    proxied_module
      .output_path
      .file_name()
      .unwrap()
      .to_string_lossy()
  );

  // for simplicity, always include the `export *` statement as it won't error
  // even when the module does not contain a named export
  writeln!(text, "export * from \"{relative_specifier}\";").unwrap();

  // add a default export if one exists in the module
  let parsed_source =
    parsed_source_cache.get_parsed_source_from_js_module(module)?;
  if has_default_export(&parsed_source) {
    writeln!(text, "export {{ default }} from \"{relative_specifier}\";")
      .unwrap();
  }

  Ok(text)
}

