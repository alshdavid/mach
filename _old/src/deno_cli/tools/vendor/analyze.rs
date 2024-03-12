// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use deno_ast::swc::ast::ExportDefaultDecl;
use deno_ast::swc::ast::ExportSpecifier;
use deno_ast::swc::ast::ModuleExportName;
use deno_ast::swc::ast::NamedExport;
use deno_ast::swc::ast::Program;
use deno_ast::swc::visit::noop_visit_type;
use deno_ast::swc::visit::Visit;
use deno_ast::swc::visit::VisitWith;
use deno_ast::ParsedSource;

/// Gets if the parsed source has a default export.
pub fn has_default_export(source: &ParsedSource) -> bool {
  let mut visitor = DefaultExportFinder {
    has_default_export: false,
  };
  let program = source.program();
  let program: &Program = &program;
  program.visit_with(&mut visitor);
  visitor.has_default_export
}

struct DefaultExportFinder {
  has_default_export: bool,
}

impl Visit for DefaultExportFinder {
  noop_visit_type!();

  fn visit_export_default_decl(&mut self, _: &ExportDefaultDecl) {
    self.has_default_export = true;
  }

  fn visit_named_export(&mut self, named_export: &NamedExport) {
    if named_export
      .specifiers
      .iter()
      .any(export_specifier_has_default)
    {
      self.has_default_export = true;
    }
  }
}

fn export_specifier_has_default(s: &ExportSpecifier) -> bool {
  match s {
    ExportSpecifier::Default(_) => true,
    ExportSpecifier::Namespace(_) => false,
    ExportSpecifier::Named(named) => {
      let export_name = named.exported.as_ref().unwrap_or(&named.orig);

      match export_name {
        ModuleExportName::Str(_) => false,
        ModuleExportName::Ident(ident) => &*ident.sym == "default",
      }
    }
  }
}

