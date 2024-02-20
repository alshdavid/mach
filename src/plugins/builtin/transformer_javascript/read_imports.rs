use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use normalize_path::NormalizePath;
use once_cell::sync::Lazy;
use swc_core::atoms::Atom;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Visit;
use swc_core::ecma::visit::VisitWith;

use crate::public::DependencyPriority;
use crate::public::ImportSymbolType;
use crate::public::SpecifierType;

static REQUIRE_SYMBOL: Lazy<Atom> = Lazy::new(|| Atom::from("require"));

#[derive(Debug)]
pub struct ImportReadResult {
  pub specifier: String,
  pub specifier_type: SpecifierType,
  pub priority: DependencyPriority,
  pub imported_symbols: Vec<ImportSymbolType>,
}

pub fn read_imports(module: &Program, file_path: &Path) -> Vec<ImportReadResult> {
  let mut w = Walker {
    imports_sync: HashMap::new(),
    imports_lazy: HashMap::new(),
    imports_require: HashMap::new(),
    file_path: file_path.parent().unwrap().to_path_buf(),
  };

  module.visit_with(&mut w);

  let mut result = Vec::<ImportReadResult>::new();

  for (specifier, imported_symbols) in w.imports_lazy {
    result.push(ImportReadResult { 
      specifier, 
      specifier_type: SpecifierType::ESM, 
      priority: DependencyPriority::Lazy, 
      imported_symbols 
    })
  }

  for (specifier, imported_symbols) in w.imports_sync {
    result.push(ImportReadResult { 
      specifier, 
      specifier_type: SpecifierType::ESM, 
      priority: DependencyPriority::Sync, 
      imported_symbols 
    })
  }

  for (specifier, imported_symbols) in w.imports_require {
    result.push(ImportReadResult { 
      specifier, 
      specifier_type: SpecifierType::Commonjs, 
      priority: DependencyPriority::Sync, 
      imported_symbols 
    })
  }

  return result;
}


#[derive(Debug)]
struct Walker {
  file_path: PathBuf,
  imports_sync: HashMap<String, Vec<ImportSymbolType>>,
  imports_lazy: HashMap<String, Vec<ImportSymbolType>>,
  imports_require: HashMap<String, Vec<ImportSymbolType>>,
}

impl Walker {
  fn normalize_specifier(&self, specifier: &str) -> String {
    if !specifier.starts_with(".") {
      return specifier.to_string();
    }

    let specifier_path = self.file_path.join(specifier).normalize();
    let relative = pathdiff::diff_paths(&specifier_path, &self.file_path).unwrap();
    let relative_str = relative.to_str().unwrap().to_string();
    return format!("./{}", relative_str);
  }

  fn insert_import_sync(&mut self, specifier: &str, import_symbol: ImportSymbolType) {
    let specifier = self.normalize_specifier(specifier);

    if let Some(imports) = self.imports_sync.get_mut(&specifier) {
      imports.push(import_symbol);
    } else {
      self.imports_sync.insert(specifier.to_string(), vec![import_symbol]);
    }
  }

  fn insert_import_lazy(&mut self, specifier: &str, import_symbol: ImportSymbolType) {
    let specifier = self.normalize_specifier(specifier);

    if let Some(imports) = self.imports_lazy.get_mut(&specifier) {
      imports.push(import_symbol);
    } else {
      self.imports_lazy.insert(specifier.to_string(), vec![import_symbol]);
    }
  }

  fn insert_import_require(&mut self, specifier: &str, import_symbol: ImportSymbolType) {
    let specifier = self.normalize_specifier(specifier);

    if let Some(imports) = self.imports_require.get_mut(&specifier) {
      imports.push(import_symbol);
    } else {
      self.imports_require.insert(specifier.to_string(), vec![import_symbol]);
    }
  }
}

impl Visit for Walker {
  // Recursive
  fn visit_module(
    &mut self,
    n: &Module,
  ) {
    n.visit_children_with(self);
  }

  // import "specifier"
  // import {} from "specifier"
  // import * as foo from "specifier"
  // import foo from "specifier"
  fn visit_import_decl(
    &mut self,
    node: &ImportDecl,
  ) {
    if node.type_only {
      return;
    }

    for specifier in &node.specifiers {      
      let import_specifier = &node.src.value.to_string();

      match &specifier {
        ImportSpecifier::Named(name) => {
          self.insert_import_sync(&import_specifier, ImportSymbolType::Named(name.local.sym.to_string()));
        },
        ImportSpecifier::Default(_) => {
          self.insert_import_sync(&import_specifier, ImportSymbolType::Default);
        },
        ImportSpecifier::Namespace(_) => {
          self.insert_import_sync(&import_specifier, ImportSymbolType::Namespace);
        },
      }
    }
  }

  // export * as foo from "specifier"
  fn visit_export_all(
    &mut self,
    node: &ExportAll,
  ) {
    if node.type_only {
      return;
    }
    let import_specifier = &node.src.value.to_string();
    self.insert_import_sync(&import_specifier, ImportSymbolType::Namespace);
  }

  // export {} from "specifier"
  fn visit_named_export(
    &mut self,
    node: &NamedExport,
  ) {
    if node.type_only {
      return;
    }
    let Some(import_specifier) = &node.src else {
      return;
    };
    let import_specifier = &import_specifier.value.to_string();

    for specifier in &node.specifiers {    
      match &specifier {
        ExportSpecifier::Namespace(_) => {
          self.insert_import_sync(&import_specifier, ImportSymbolType::Namespace);
        }
        ExportSpecifier::Default(_) => {
          self.insert_import_sync(&import_specifier, ImportSymbolType::Default);
        }
        ExportSpecifier::Named(name) => {
          match &name.orig {
            ModuleExportName::Ident(ident) => {
              self.insert_import_sync(&import_specifier, ImportSymbolType::Named(ident.sym.to_string()));
            },
            ModuleExportName::Str(_) => todo!(),
          }
        }
      }
    }
  }

  // import("specifier")
  // require("specifier")
  fn visit_call_expr(
    &mut self,
    node: &CallExpr,
  ) {
    node.visit_children_with(self);

    match &node.callee {
      // import("specifier")
      Callee::Import(_) => {
        if node.args.len() == 0 {
          return;
        }
        let Expr::Lit(import_specifier_arg) = &*node.args[0].expr else {
          return;
        };
        let Lit::Str(import_specifier) = import_specifier_arg else {
          return;
        };
        self.insert_import_lazy(&import_specifier.value.to_string(), ImportSymbolType::Namespace);
      }
      // require("specifier")
      Callee::Expr(expr) => {
        let Expr::Ident(ident) = &**expr else {
          return;
        };
        if ident.sym != *REQUIRE_SYMBOL {
          return;
        }
        let Expr::Lit(import_specifier_arg) = &*node.args[0].expr else {
          return;
        };
        let Lit::Str(import_specifier) = import_specifier_arg else {
          return;
        };
        self.insert_import_require(&import_specifier.value.to_string(), ImportSymbolType::Namespace);
      }
      Callee::Super(_) => {}
    }
  }
}
