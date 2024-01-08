use swc_core::ecma::ast::*;

use crate::bundle::BundleDependencyIndex;

/*
  const foo = ''; export { foo }
  const foo = ''; export { foo as bar }
*/
pub fn read_exports_named(
  decl: &NamedExport,
  asset_id: &str,
  dependency_index: &BundleDependencyIndex,
) -> ExportAssignment {
  // If export contains a ~ from './specifier' ~, lookup the asset_id of the import specifier
  let asset_id: Option<String> = {
    if let Some(src) = &decl.src {
      let specifier = &src.value.to_string();
      let (asset_id, _) = dependency_index
        .get(&(asset_id.to_string(), specifier.clone()))
        .unwrap();
      Some(asset_id.clone())
    } else {
      None
    }
  };

  let mut export_assignments = Vec::<ExportNamedAssignment>::new();

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
            export_assignments.push(ExportNamedAssignment::RenamedKey(export_name, exported_as));
          }
        } else {
          export_assignments.push(ExportNamedAssignment::NamedKey(export_name));
        }
      }
      // export * as foo from './foo'
      //
      // I could be wrong, but a namespace asset can only have one assignment so
      // I assume decl.specifiers.len() will be 0
      ExportSpecifier::Namespace(decl) => {
        let Some(asset_id) = asset_id else {
          panic!("Invalid namespace export");
        };

        let ModuleExportName::Ident(ident) = &decl.name else {
          panic!("Invalid namespace export");
        };

        return ExportAssignment::ReexportNamespace(asset_id, ident.sym.to_string());
      }
      _ => {
        panic!("Not Implemented")
      }
    }
  }

  if let Some(asset_id) = asset_id {
    return ExportAssignment::ReexportNamed(asset_id, export_assignments);
  }

  return ExportAssignment::ExportNamed(export_assignments);
}

pub enum ExportAssignment {
  ReexportNamed(String, Vec<ExportNamedAssignment>),
  ReexportNamespace(String, String),
  ExportNamed(Vec<ExportNamedAssignment>),
}

pub enum ExportNamedAssignment {
  /// export { foo }
  ///          ---
  NamedKey(String),
  /// export { foo as bar }'
  ///          ----------
  RenamedKey(String, String),
}
