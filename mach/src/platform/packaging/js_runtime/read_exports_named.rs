use swc_core::ecma::ast::*;

use crate::platform::packaging::runtime_factory::ExportNamed;

/*
  const foo = ''; export { foo }
  const foo = ''; export { foo as bar }
*/
pub fn read_exports_named(
  decl: NamedExport,
  import_specifier: Option<String>,
) -> ExportAssignment {
  let mut export_assignments = Vec::<ExportNamed>::new();

  for specifier in &decl.specifiers {
    match specifier {
      ExportSpecifier::Named(decl) => {
        let ModuleExportName::Ident(ident) = &decl.orig else {
          panic!("Invalid export export");
        };

        let export_name = ident.sym.to_string();

        if let Some(decl_export) = &decl.exported {
          if let ModuleExportName::Ident(ident) = &decl_export {
            let exported_as = ident.sym.to_string();
            export_assignments.push(ExportNamed::Renamed(export_name, exported_as));
          }
        } else {
          export_assignments.push(ExportNamed::Named(export_name));
        }
      }
      // export * as foo from './foo'
      //
      // I could be wrong, but a namespace asset can only have one assignment so
      // I assume decl.specifiers.len() will be 0
      ExportSpecifier::Namespace(decl) => {
        let Some(import_specifier) = import_specifier else {
          panic!("Invalid namespace export");
        };

        let ModuleExportName::Ident(ident) = &decl.name else {
          panic!("Invalid namespace export");
        };

        return ExportAssignment::ReexportNamespace(ident.sym.to_string(), import_specifier);
      }
      _ => {
        panic!("Not Implemented")
      }
    }
  }

  if let Some(import_specifier) = import_specifier {
    return ExportAssignment::ReexportNamed(export_assignments, import_specifier);
  }

  return ExportAssignment::ExportNamed(export_assignments);
}

#[derive(Debug)]
pub enum ExportAssignment {
  /// export { attributes } from './specifier'
  ReexportNamed(Vec<ExportNamed>, String),
  /// export * as foo from './specifier'
  ReexportNamespace(String, String),
  /// export { a, b as c }
  ExportNamed(Vec<ExportNamed>),
}
