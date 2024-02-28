use swc_core::ecma::ast::*;

use crate::packaging::runtime_factory::ImportNamed;

/// ImportAssignment looks at the variables assigned within an import statement
///   import { a, b } from './foo'
///          |------| <-- This part
pub fn read_import_assignments(import_decl: &ImportDecl) -> ImportAssignment {
  // This is an import without assignments
  // import './foo'
  if import_decl.specifiers.len() == 0 {
    return ImportAssignment::None;
  }

  // This is a star import
  // import * as foo from './foo'
  if let ImportSpecifier::Namespace(decl) = &import_decl.specifiers[0] {
    return ImportAssignment::Star(decl.local.sym.to_string());
  }

  // This is a named or default import
  // import { foo } from './foo'
  // import { foo as bar } from './foo'
  // import foo from './foo'
  // import foo, { bar } from './foo'
  let mut named_assignments = Vec::<ImportNamed>::new();

  for specifier in import_decl.specifiers.iter() {
    match specifier {
      // This is a named import
      // import { foo } from './foo'
      ImportSpecifier::Named(import_specifier) => {
        if let Some(imported) = &import_specifier.imported {
          // This is an aliased named import
          // import { foo as bar } from './foo'
          if let ModuleExportName::Ident(ident) = imported {
            let ident_str = ident.sym.to_string();
            if ident_str != import_specifier.local.sym.to_string() {
              named_assignments.push(ImportNamed::Renamed(
                import_specifier.local.sym.to_string(),
                ident_str,
              ));
              continue;
            }
          }
        }
        named_assignments.push(ImportNamed::Named(import_specifier.local.sym.to_string()));
      }
      ImportSpecifier::Default(import_specifier) => {
        // This is a default import
        // import './foo'
        named_assignments.push(ImportNamed::Default(import_specifier.local.sym.to_string()));
      }
      _ => {}
    }
  }

  return ImportAssignment::Named(named_assignments);
}

pub enum ImportAssignment {
  /// FROM: import * as foo from './foo'
  /// TO:   "foo"
  Star(String),
  /// FROM: import { a, b } from './foo'
  /// TO:   [NamedImport::Key("a"), NamedImport::Key("b")]
  ///
  /// FROM: import { a, b as c} from './foo'
  /// TO:   [NamedImport::Key("a"), NamedImport::Renamed("b", "c")]
  ///
  /// FROM: import a, { b } from './foo'
  /// TO:   [NamedImport::Default("a"), NamedImport::Key("b")]
  Named(Vec<ImportNamed>),
  /// FROM: import './foo'
  /// TO:   None
  None,
}