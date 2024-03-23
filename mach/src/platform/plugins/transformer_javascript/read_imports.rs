use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

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

pub fn read_imports_exports(
  module: &Program,
  file_path: &Path,
) -> (Vec<ImportReadResult>, Vec<ExportSymbol>) {
  let mut w = Walker {
    imports_sync: HashMap::new(),
    imports_lazy: HashMap::new(),
    imports_require: HashMap::new(),
    _file_path: file_path.parent().unwrap().to_path_buf(),
    exports: Vec::<ExportSymbol>::new(),
  };

  module.visit_with(&mut w);

  let mut result = Vec::<ImportReadResult>::new();

  for (specifier, imported_symbols) in w.imports_lazy {
    result.push(ImportReadResult {
      specifier,
      specifier_type: SpecifierType::ESM,
      priority: DependencyPriority::Lazy,
      imported_symbols,
    })
  }

  for (specifier, imported_symbols) in w.imports_sync {
    result.push(ImportReadResult {
      specifier,
      specifier_type: SpecifierType::ESM,
      priority: DependencyPriority::Sync,
      imported_symbols,
    })
  }

  for (specifier, imported_symbols) in w.imports_require {
    result.push(ImportReadResult {
      specifier,
      specifier_type: SpecifierType::Commonjs,
      priority: DependencyPriority::Sync,
      imported_symbols,
    })
  }

  return (result, w.exports);
}

#[derive(Debug)]
struct Walker {
  _file_path: PathBuf,
  imports_sync: HashMap<String, Vec<ImportSymbolType>>,
  imports_lazy: HashMap<String, Vec<ImportSymbolType>>,
  imports_require: HashMap<String, Vec<ImportSymbolType>>,
  exports: Vec<ExportSymbol>,
}

impl Walker {
  fn normalize_specifier(
    &self,
    specifier: &str,
  ) -> String {
    if !specifier.starts_with(".") {
      return specifier.to_string();
    }

    return specifier.to_string();
  }

  fn insert_import_sync(
    &mut self,
    specifier: &str,
    import_symbol: ImportSymbolType,
  ) {
    let specifier = self.normalize_specifier(specifier);

    if let Some(imports) = self.imports_sync.get_mut(&specifier) {
      imports.push(import_symbol);
    } else {
      self
        .imports_sync
        .insert(specifier.to_string(), vec![import_symbol]);
    }
  }

  fn insert_import_lazy(
    &mut self,
    specifier: &str,
    import_symbol: ImportSymbolType,
  ) {
    let specifier = self.normalize_specifier(specifier);

    if let Some(imports) = self.imports_lazy.get_mut(&specifier) {
      imports.push(import_symbol);
    } else {
      self
        .imports_lazy
        .insert(specifier.to_string(), vec![import_symbol]);
    }
  }

  fn insert_import_require(
    &mut self,
    specifier: &str,
    import_symbol: ImportSymbolType,
  ) {
    let specifier = self.normalize_specifier(specifier);

    if let Some(imports) = self.imports_require.get_mut(&specifier) {
      imports.push(import_symbol);
    } else {
      self
        .imports_require
        .insert(specifier.to_string(), vec![import_symbol]);
    }
  }
}

impl Visit for Walker {
  fn visit_module(
    &mut self,
    module: &Module,
  ) {
    module.visit_children_with(self);

    'module_loop: for i in 0..module.body.len() {
      match &module.body[i] {
        ModuleItem::ModuleDecl(decl) => {
          match &decl {
            // import "specifier"
            // import {} from "specifier"
            // import * as foo from "specifier"
            // import foo from "specifier"
            ModuleDecl::Import(decl) => {
              if decl.type_only {
                continue 'module_loop;
              }

              let import_specifier = &decl.src.value.to_string();

              // import './foo'
              if decl.specifiers.len() == 0 {
                self.insert_import_sync(import_specifier, ImportSymbolType::Unnamed);
                continue 'module_loop;
              }

              for specifier in &decl.specifiers {
                match &specifier {
                  // import { foo } from './foo'
                  ImportSpecifier::Named(name) => {
                    self.insert_import_sync(
                      import_specifier,
                      ImportSymbolType::Named(name.local.sym.to_string()),
                    );
                  }
                  // import foo from './foo'
                  ImportSpecifier::Default(_) => {
                    self.insert_import_sync(import_specifier, ImportSymbolType::Default);
                  }
                  // import * as foo from './foo'
                  ImportSpecifier::Namespace(decl) => {
                    self.insert_import_sync(
                      import_specifier,
                      ImportSymbolType::Namespace(decl.local.sym.to_string()),
                    );
                  }
                }
              }
            }

            // export const foo = ''
            // export const { foo } = foo
            // export const [ foo ] = foo
            // export function foo() {}
            // export class foo {}
            ModuleDecl::ExportDecl(decl) => {
              match &decl.decl {
                Decl::Class(decl) => {
                  self
                    .exports
                    .push(ExportSymbol::Named(decl.ident.sym.to_string()));
                }
                Decl::Fn(decl) => {
                  self
                    .exports
                    .push(ExportSymbol::Named(decl.ident.sym.to_string()));
                }
                Decl::Var(decl) => {
                  for decl in decl.decls.iter() {
                    match &decl.name {
                      // export const foo = ''
                      Pat::Ident(decl) => {
                        self
                          .exports
                          .push(ExportSymbol::Named(decl.id.sym.to_string()));
                      }
                      // export const { foo } = foo
                      Pat::Object(decl) => {
                        dbg!(&decl);
                        for prop in &decl.props {
                          match prop {
                            ObjectPatProp::Assign(prop) => {
                              self
                                .exports
                                .push(ExportSymbol::Named(prop.key.sym.to_string()));
                            }
                            // export const { foo: bar } = { foo: 'text' }
                            //   essentially "foo" as "bar"
                            ObjectPatProp::KeyValue(prop) => {
                              let Some(ident) = prop.value.as_ident() else { todo!() };
                              self
                                .exports
                                .push(ExportSymbol::Named(ident.sym.to_string()));
                            },
                            ObjectPatProp::Rest(_) => todo!(),
                          }
                        }
                      }
                      // export const [ foo ] = foo
                      Pat::Array(decl) => {
                        for elm in &decl.elems {
                          let Some(elm) = elm else {
                            continue;
                          };
                          match elm {
                            Pat::Ident(ident) => {
                              self
                                .exports
                                .push(ExportSymbol::Named(ident.sym.to_string()));
                            }
                            Pat::Array(_) => todo!(),
                            Pat::Rest(_) => todo!(),
                            Pat::Object(_) => todo!(),
                            Pat::Assign(_) => todo!(),
                            Pat::Invalid(_) => todo!(),
                            Pat::Expr(_) => todo!(),
                          }
                        }
                      }
                      Pat::Rest(_) => panic!("Should not happen"),
                      Pat::Assign(_) => panic!("Should not happen"),
                      Pat::Invalid(_) => panic!("Should not happen"),
                      Pat::Expr(_) => panic!("Should not happen"),
                    };
                  }
                }
                Decl::Using(_) => todo!(),
                Decl::TsInterface(_) => panic!("Should not see TS"),
                Decl::TsTypeAlias(_) => panic!("Should not see TS"),
                Decl::TsEnum(_) => panic!("Should not see TS"),
                Decl::TsModule(_) => panic!("Should not see TS"),
              }
            }

            // export * as foo from "specifier"
            // export { foo } from "specifier"
            ModuleDecl::ExportNamed(decl) => {
              if decl.type_only {
                continue 'module_loop;
              }
              let Some(import_specifier) = &decl.src else {
                continue 'module_loop;
              };
              let import_specifier = &import_specifier.value.to_string();

              for specifier in &decl.specifiers {
                match &specifier {
                  // export * as foo from "specifier"
                  ExportSpecifier::Namespace(name) => {
                    match &name.name {
                      ModuleExportName::Ident(ident) => {
                        self.insert_import_sync(
                          &import_specifier,
                          ImportSymbolType::Namespace(ident.sym.to_string()),
                        );

                        self
                          .exports
                          .push(ExportSymbol::Named(ident.sym.to_string()));
                      }
                      ModuleExportName::Str(_) => todo!(),
                    };
                  }
                  ExportSpecifier::Default(_) => {
                    self.insert_import_sync(&import_specifier, ImportSymbolType::Default);
                    self.exports.push(ExportSymbol::Default);
                    panic!("I don't think this can happen");
                  }
                  // export { foo } from "specifier"
                  ExportSpecifier::Named(name) => match &name.orig {
                    ModuleExportName::Ident(ident) => {
                      self.insert_import_sync(
                        &import_specifier,
                        ImportSymbolType::Named(ident.sym.to_string()),
                      );
                    }
                    ModuleExportName::Str(_) => todo!(),
                  },
                }
              }
            }

            // export default class foo {}
            // export default class {}
            // export default function foo() {}
            // export default function() {}
            ModuleDecl::ExportDefaultDecl(_) => {
              self.exports.push(ExportSymbol::Default);
            }

            // export default ''
            ModuleDecl::ExportDefaultExpr(_) => {
              self.exports.push(ExportSymbol::Default);
            }

            // export * from "specifier"
            ModuleDecl::ExportAll(decl) => {
              if decl.type_only {
                continue 'module_loop;
              }
              // dbg!(&decl);
              let import_specifier = decl.src.value.to_string();
              self.insert_import_sync(&import_specifier, ImportSymbolType::Reexport);
              self.exports.push(ExportSymbol::ExportAll(import_specifier));
            }
            ModuleDecl::TsImportEquals(_) => panic!("Should not see TS"),
            ModuleDecl::TsExportAssignment(_) => panic!("Should not see TS"),
            ModuleDecl::TsNamespaceExport(_) => panic!("Should not see TS"),
          }
        }
        ModuleItem::Stmt(_) => {}
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
        self.insert_import_lazy(
          &import_specifier.value.to_string(),
          ImportSymbolType::Dynamic,
        );
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
        self.insert_import_require(
          &import_specifier.value.to_string(),
          ImportSymbolType::Commonjs,
        );
      }
      Callee::Super(_) => {}
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ExportSymbol {
  // export const foo = ''
  // export const { foo, bar } = foobar
  //               |---||---|
  // export { foo }
  // export { foo as bar }
  //                |---|
  Named(String),
  // export default foo
  Default,
  // export * from './foo'
  ExportAll(String),
}
