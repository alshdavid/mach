// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use deno_ast::ParsedSource;
use deno_ast::SourceRange;
use deno_ast::SourceTextInfo;
use deno_core::ModuleSpecifier;
use deno_graph::DefaultModuleAnalyzer;
use deno_graph::DependencyDescriptor;
use deno_graph::DynamicTemplatePart;
use deno_graph::TypeScriptReference;
use deno_runtime::deno_node::is_builtin_node_module;

use crate::deno_cli::resolver::MappedSpecifierResolver;
use crate::deno_cli::resolver::SloppyImportsResolver;

#[derive(Debug, Clone)]
pub enum SpecifierUnfurlerDiagnostic {
  UnanalyzableDynamicImport {
    specifier: ModuleSpecifier,
    text_info: SourceTextInfo,
    range: SourceRange,
  },
}

impl SpecifierUnfurlerDiagnostic {
  pub fn code(&self) -> &'static str {
    match self {
      Self::UnanalyzableDynamicImport { .. } => "unanalyzable-dynamic-import",
    }
  }

  pub fn message(&self) -> &'static str {
    match self {
      Self::UnanalyzableDynamicImport { .. } => "unable to analyze dynamic import",
    }
  }
}

pub struct SpecifierUnfurler<'a> {
  mapped_resolver: &'a MappedSpecifierResolver,
  sloppy_imports_resolver: Option<&'a SloppyImportsResolver>,
  bare_node_builtins: bool,
}

impl<'a> SpecifierUnfurler<'a> {
  pub fn new(
    mapped_resolver: &'a MappedSpecifierResolver,
    sloppy_imports_resolver: Option<&'a SloppyImportsResolver>,
    bare_node_builtins: bool,
  ) -> Self {
    Self {
      mapped_resolver,
      sloppy_imports_resolver,
      bare_node_builtins,
    }
  }

  fn unfurl_specifier(
    &self,
    referrer: &ModuleSpecifier,
    specifier: &str,
  ) -> Option<String> {
    let resolved = if let Ok(resolved) = self.mapped_resolver.resolve(specifier, referrer) {
      resolved.into_specifier()
    } else {
      None
    };
    let resolved = match resolved {
      Some(resolved) => resolved,
      None if self.bare_node_builtins && is_builtin_node_module(specifier) => {
        format!("node:{specifier}").parse().unwrap()
      }
      None => ModuleSpecifier::options()
        .base_url(Some(referrer))
        .parse(specifier)
        .ok()?,
    };
    // TODO(lucacasonato): this requires integration in deno_graph first
    // let resolved = if let Ok(specifier) =
    //   NpmPackageReqReference::from_specifier(&resolved)
    // {
    //   if let Some(scope_name) = specifier.req().name.strip_prefix("@jsr/") {
    //     let (scope, name) = scope_name.split_once("__")?;
    //     let new_specifier = JsrPackageReqReference::new(PackageReqReference {
    //       req: PackageReq {
    //         name: format!("@{scope}/{name}"),
    //         version_req: specifier.req().version_req.clone(),
    //       },
    //       sub_path: specifier.sub_path().map(ToOwned::to_owned),
    //     })
    //     .to_string();
    //     ModuleSpecifier::parse(&new_specifier).unwrap()
    //   } else {
    //     resolved
    //   }
    // } else {
    //   resolved
    // };
    let resolved = if let Some(sloppy_imports_resolver) = self.sloppy_imports_resolver {
      sloppy_imports_resolver
        .resolve(&resolved, deno_graph::source::ResolutionMode::Execution)
        .as_specifier()
        .clone()
    } else {
      resolved
    };
    let relative_resolved = relative_url(&resolved, referrer);
    if relative_resolved == specifier {
      None // nothing to unfurl
    } else {
      Some(relative_resolved)
    }
  }

  /// Attempts to unfurl the dynamic dependency returning `true` on success
  /// or `false` when the import was not analyzable.
  fn try_unfurl_dynamic_dep(
    &self,
    module_url: &lsp_types::Url,
    parsed_source: &ParsedSource,
    dep: &deno_graph::DynamicDependencyDescriptor,
    text_changes: &mut Vec<deno_ast::TextChange>,
  ) -> bool {
    match &dep.argument {
      deno_graph::DynamicArgument::String(specifier) => {
        let range = to_range(parsed_source, &dep.argument_range);
        let maybe_relative_index =
          parsed_source.text_info().text_str()[range.start..range.end].find(specifier);
        let Some(relative_index) = maybe_relative_index else {
          return true; // always say it's analyzable for a string
        };
        let unfurled = self.unfurl_specifier(module_url, specifier);
        if let Some(unfurled) = unfurled {
          let start = range.start + relative_index;
          text_changes.push(deno_ast::TextChange {
            range: start..start + specifier.len(),
            new_text: unfurled,
          });
        }
        true
      }
      deno_graph::DynamicArgument::Template(parts) => match parts.first() {
        Some(DynamicTemplatePart::String { value: specifier }) => {
          // relative doesn't need to be modified
          let is_relative = specifier.starts_with("./") || specifier.starts_with("../");
          if is_relative {
            return true;
          }
          if !specifier.ends_with('/') {
            return false;
          }
          let unfurled = self.unfurl_specifier(module_url, specifier);
          let Some(unfurled) = unfurled else {
            return true; // nothing to unfurl
          };
          let range = to_range(parsed_source, &dep.argument_range);
          let maybe_relative_index =
            parsed_source.text_info().text_str()[range.start..].find(specifier);
          let Some(relative_index) = maybe_relative_index else {
            return false;
          };
          let start = range.start + relative_index;
          text_changes.push(deno_ast::TextChange {
            range: start..start + specifier.len(),
            new_text: unfurled,
          });
          true
        }
        Some(DynamicTemplatePart::Expr) => {
          false // failed analyzing
        }
        None => {
          true // ignore
        }
      },
      deno_graph::DynamicArgument::Expr => {
        false // failed analyzing
      }
    }
  }

  pub fn unfurl(
    &self,
    url: &ModuleSpecifier,
    parsed_source: &ParsedSource,
    diagnostic_reporter: &mut dyn FnMut(SpecifierUnfurlerDiagnostic),
  ) -> String {
    let mut text_changes = Vec::new();
    let module_info = DefaultModuleAnalyzer::module_info(parsed_source);
    let analyze_specifier =
      |specifier: &str,
       range: &deno_graph::PositionRange,
       text_changes: &mut Vec<deno_ast::TextChange>| {
        if let Some(unfurled) = self.unfurl_specifier(url, specifier) {
          text_changes.push(deno_ast::TextChange {
            range: to_range(parsed_source, range),
            new_text: unfurled,
          });
        }
      };
    for dep in &module_info.dependencies {
      match dep {
        DependencyDescriptor::Static(dep) => {
          analyze_specifier(&dep.specifier, &dep.specifier_range, &mut text_changes);
        }
        DependencyDescriptor::Dynamic(dep) => {
          let success = self.try_unfurl_dynamic_dep(url, parsed_source, dep, &mut text_changes);

          if !success {
            let start_pos = parsed_source
              .text_info()
              .line_start(dep.argument_range.start.line)
              + dep.argument_range.start.character;
            let end_pos = parsed_source
              .text_info()
              .line_start(dep.argument_range.end.line)
              + dep.argument_range.end.character;
            diagnostic_reporter(SpecifierUnfurlerDiagnostic::UnanalyzableDynamicImport {
              specifier: url.to_owned(),
              range: SourceRange::new(start_pos, end_pos),
              text_info: parsed_source.text_info().clone(),
            });
          }
        }
      }
    }
    for ts_ref in &module_info.ts_references {
      let specifier_with_range = match ts_ref {
        TypeScriptReference::Path(range) => range,
        TypeScriptReference::Types(range) => range,
      };
      analyze_specifier(
        &specifier_with_range.text,
        &specifier_with_range.range,
        &mut text_changes,
      );
    }
    for specifier_with_range in &module_info.jsdoc_imports {
      analyze_specifier(
        &specifier_with_range.text,
        &specifier_with_range.range,
        &mut text_changes,
      );
    }
    if let Some(specifier_with_range) = &module_info.jsx_import_source {
      analyze_specifier(
        &specifier_with_range.text,
        &specifier_with_range.range,
        &mut text_changes,
      );
    }

    let rewritten_text =
      deno_ast::apply_text_changes(parsed_source.text_info().text_str(), text_changes);
    rewritten_text
  }
}

fn relative_url(
  resolved: &ModuleSpecifier,
  referrer: &ModuleSpecifier,
) -> String {
  if resolved.scheme() == "file" {
    let relative = referrer.make_relative(resolved).unwrap();
    if relative.is_empty() {
      let last = resolved.path_segments().unwrap().last().unwrap();
      format!("./{last}")
    } else if relative.starts_with("../") {
      relative
    } else {
      format!("./{relative}")
    }
  } else {
    resolved.to_string()
  }
}

fn to_range(
  parsed_source: &ParsedSource,
  range: &deno_graph::PositionRange,
) -> std::ops::Range<usize> {
  let mut range = range
    .as_source_range(parsed_source.text_info())
    .as_byte_range(parsed_source.text_info().range().start);
  let text = &parsed_source.text_info().text_str()[range.clone()];
  if text.starts_with('"') || text.starts_with('\'') {
    range.start += 1;
  }
  if text.ends_with('"') || text.ends_with('\'') {
    range.end -= 1;
  }
  range
}
