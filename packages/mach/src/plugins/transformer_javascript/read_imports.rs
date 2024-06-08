use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use swc_core::atoms::Atom;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Visit;
use swc_core::ecma::visit::VisitWith;

use crate::public::DependencyPriority;
use crate::public::ExportSymbol;
use crate::public::ImportSymbol;
use crate::public::ReexportSymbol;
use crate::public::SpecifierType;

static REQUIRE_SYMBOL: Lazy<Atom> = Lazy::new(|| Atom::from("require"));

#[derive(Debug)]
pub struct ImportReadResult {
  pub specifier: String,
  pub specifier_type: SpecifierType,
  pub priority: DependencyPriority,
  pub imported_symbols: Vec<ImportSymbol>,
  pub reexported_symbols: Vec<ReexportSymbol>,
}

pub fn read_imports_exports(
  module: &Program,
  file_path: &Path,
) -> (Vec<ImportReadResult>, Vec<ExportSymbol>) {
  let mut w = Walker {
    imports_sync: HashMap::new(),
    imports_lazy: HashMap::new(),
    imports_require: HashMap::new(),
    reexports: HashMap::new(),
    _file_path: file_path.parent().unwrap().to_path_buf(),
    exports: Vec::<ExportSymbol>::new(),
  };

  // dbg!(&module);
  module.visit_with(&mut w);

  let mut result = Vec::<ImportReadResult>::new();

  for (specifier, imported_symbols) in w.imports_lazy {
    result.push(ImportReadResult {
      specifier,
      specifier_type: SpecifierType::ESM,
      priority: DependencyPriority::Lazy,
      imported_symbols,
      reexported_symbols: vec![],
    })
  }

  for (specifier, imported_symbols) in w.imports_sync {
    result.push(ImportReadResult {
      specifier,
      specifier_type: SpecifierType::ESM,
      priority: DependencyPriority::Sync,
      imported_symbols,
      reexported_symbols: vec![],
    })
  }

  for (specifier, imported_symbols) in w.imports_require {
    result.push(ImportReadResult {
      specifier,
      specifier_type: SpecifierType::Commonjs,
      priority: DependencyPriority::Sync,
      imported_symbols,
      reexported_symbols: vec![],
    })
  }

  for (specifier, reexported_symbols) in w.reexports {
    result.push(ImportReadResult {
      specifier,
      specifier_type: SpecifierType::Commonjs,
      priority: DependencyPriority::Sync,
      imported_symbols: vec![],
      reexported_symbols,
    })
  }

  return (result, w.exports);
}

#[derive(Debug)]
struct Walker {
  _file_path: PathBuf,
  imports_sync: HashMap<String, Vec<ImportSymbol>>,
  imports_lazy: HashMap<String, Vec<ImportSymbol>>,
  reexports: HashMap<String, Vec<ReexportSymbol>>,
  imports_require: HashMap<String, Vec<ImportSymbol>>,
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
    import_symbol: ImportSymbol,
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

  fn insert_reexport(
    &mut self,
    specifier: &str,
    reexport_symbol: ReexportSymbol,
  ) {
    let specifier = self.normalize_specifier(specifier);

    if let Some(imports) = self.reexports.get_mut(&specifier) {
      imports.push(reexport_symbol);
    } else {
      self
        .reexports
        .insert(specifier.to_string(), vec![reexport_symbol]);
    }
  }

  fn insert_import_lazy(
    &mut self,
    specifier: &str,
    import_symbol: ImportSymbol,
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
    import_symbol: ImportSymbol,
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
      // dbg!(&module.body[i]);

      match &module.body[i] {
        ModuleItem::ModuleDecl(decl) => {
          match &decl {
            // import "specifier"
            // import { foo } from "specifier"
            // import { foo as bar } from "specifier"
            // import * as foo from "specifier"
            // import foo from "specifier"
            ModuleDecl::Import(decl) => {
              if decl.type_only {
                continue 'module_loop;
              }

              let import_specifier = &decl.src.value.to_string();

              // import './foo'
              if decl.specifiers.len() == 0 {
                self.insert_import_sync(import_specifier, ImportSymbol::Unnamed);
                continue 'module_loop;
              }

              for specifier in &decl.specifiers {
                match &specifier {
                  // import { foo } from './foo'
                  // import { foo as bar } from './foo'
                  ImportSpecifier::Named(name) => {
                    // import { foo as bar } from './foo'
                    if let Some(imported) = &name.imported {
                      let ModuleExportName::Ident(imported) = imported else {
                        unreachable!();
                      };
                      self.insert_import_sync(
                        import_specifier,
                        ImportSymbol::Renamed {
                          sym: imported.sym.to_string(),
                          sym_as: name.local.sym.to_string(),
                        },
                      );
                    } else {
                      // import { foo } from './foo'
                      self.insert_import_sync(
                        import_specifier,
                        ImportSymbol::Named {
                          sym: name.local.sym.to_string(),
                        },
                      );
                    }
                  }
                  // import foo from './foo'
                  ImportSpecifier::Default(ident) => {
                    self.insert_import_sync(
                      import_specifier,
                      ImportSymbol::Default {
                        sym_as: ident.local.sym.to_string(),
                      },
                    );
                  }
                  // import * as foo from './foo'
                  ImportSpecifier::Namespace(decl) => {
                    self.insert_import_sync(
                      import_specifier,
                      ImportSymbol::Namespace {
                        sym_as: decl.local.sym.to_string(),
                      },
                    );
                  }
                }
              }
            }

            // export const { foo } = foo
            // export const [ foo ] = foo
            ModuleDecl::ExportDecl(decl) => {
              match &decl.decl {
                //
                // export class foo {}
                //
                Decl::Class(decl) => {
                  self.exports.push(ExportSymbol::Named {
                    sym: decl.ident.sym.to_string(),
                  });
                }
                //
                // export function foo() {}
                //
                Decl::Fn(decl) => {
                  self.exports.push(ExportSymbol::Named {
                    sym: decl.ident.sym.to_string(),
                  });
                }

                Decl::Var(decl) => {
                  dbg!(&decl);

                  for var_decl in decl.decls.iter() {
                    match &var_decl.name {
                      //
                      // export const foo = ''
                      //
                      Pat::Ident(decl) => {
                        self.exports.push(ExportSymbol::Named {
                          sym: decl.id.sym.to_string(),
                        });
                      }

                      Pat::Object(decl) => {
                        let Some(sym_source) = &var_decl.init else {
                          panic!();
                        };
                        let Expr::Ident(sym_source) = &**sym_source else {
                          panic!();
                        };

                        for prop in &decl.props {
                          match prop {
                            //
                            // export const { one } = foo
                            //
                            ObjectPatProp::Assign(prop) => {
                              self.exports.push(ExportSymbol::Destructured {
                                sym: prop.key.sym.to_string(),
                                sym_source: sym_source.sym.to_string(),
                              });
                            }
                            //
                            // export const { one: two } = foo
                            // export const { ['one']: two } = foo
                            // export const { [1]: two } = foo
                            //
                            ObjectPatProp::KeyValue(prop) => {
                              let Some(ident) = prop.value.as_ident() else {
                                todo!()
                              };
                              let key = match &prop.key {
                                PropName::Ident(ident) => ident.sym.to_string(),
                                PropName::Str(str) => str.value.to_string(),
                                PropName::Num(num) => num.value.to_string(),
                                PropName::Computed(computed) => match &*computed.expr {
                                  Expr::Ident(ident) => ident.sym.to_string(),
                                  Expr::Lit(lit) => match lit {
                                    Lit::Str(str) => str.value.to_string(),
                                    Lit::Num(num) => num.value.to_string(),
                                    _ => todo!(),
                                  },
                                  _ => todo!(),
                                },
                                PropName::BigInt(_) => todo!(),
                              };
                              self.exports.push(ExportSymbol::DestructuredRenamed {
                                sym: key,
                                sym_as: ident.sym.to_string(),
                                sym_source: sym_source.sym.to_string(),
                              });
                            }
                            ObjectPatProp::Rest(_) => todo!(),
                          }
                        }
                      }
                      //
                      // export const [ foo ] = foo
                      // TODO
                      //    export const [{ foo }] = obj
                      //    export const [[{ foo }]] = obj
                      //    export const [[ foo ]] = obj
                      Pat::Array(decl) => {
                        let Some(sym_source) = &var_decl.init else {
                          panic!();
                        };
                        let Expr::Ident(sym_source) = &**sym_source else {
                          panic!();
                        };

                        for elm in &decl.elems {
                          let Some(elm) = elm else {
                            continue;
                          };
                          match elm {
                            Pat::Ident(ident) => {
                              self.exports.push(ExportSymbol::Destructured {
                                sym: ident.sym.to_string(),
                                sym_source: sym_source.sym.to_string(),
                              });
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

              for specifier in &decl.specifiers {
                match &specifier {
                  // export * as foo from "specifier"
                  ExportSpecifier::Namespace(name) => {
                    match &name.name {
                      ModuleExportName::Ident(ident) => {
                        let Some(import_specifier) = &decl.src else {
                          continue 'module_loop;
                        };
                        let import_specifier = &import_specifier.value.to_string();

                        self.insert_import_sync(
                          &import_specifier,
                          ImportSymbol::Namespace {
                            sym_as: ident.sym.to_string(),
                          },
                        );

                        self.exports.push(ExportSymbol::Named {
                          sym: ident.sym.to_string(),
                        });
                      }
                      ModuleExportName::Str(_) => todo!(),
                    };
                  }
                  // I don't think this can happen
                  ExportSpecifier::Default(_) => {
                    unreachable!()
                  }
                  ExportSpecifier::Named(name) => match &name.orig {
                    ModuleExportName::Ident(ident) => {
                      if let Some(import_specifier) = &decl.src {
                        let import_specifier = &import_specifier.value.to_string();
                        //
                        // export { foo as bar } from "specifier"
                        //
                        if let Some(ModuleExportName::Ident(exported)) = &name.exported {
                          self.insert_reexport(
                            import_specifier,
                            ReexportSymbol::Renamed {
                              sym: ident.sym.to_string(),
                              sym_as: exported.sym.to_string(),
                            },
                          );
                        }
                        //
                        // export { foo } from "specifier"
                        //
                        else {
                          self.insert_reexport(
                            import_specifier,
                            ReexportSymbol::Named {
                              sym: ident.sym.to_string(),
                            },
                          );
                        }
                      } else {
                        //
                        // export { foo as bar }"
                        //
                        if let Some(ModuleExportName::Ident(exported)) = &name.exported {
                          self.exports.push(ExportSymbol::Renamed {
                            sym: ident.sym.to_string(),
                            sym_as: exported.sym.to_string(),
                          });
                        }
                        //
                        // export { foo }"
                        //
                        else {
                          self.exports.push(ExportSymbol::Named {
                            sym: ident.sym.to_string(),
                          });
                        }
                      };
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
              // self.insert_import_sync(&import_specifier, ImportSymbol::Reexport);
              // self.exports.push(ExportSymbol::ExportAll(import_specifier));
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
        self.insert_import_lazy(&import_specifier.value.to_string(), ImportSymbol::Dynamic);
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
        self.insert_import_require(&import_specifier.value.to_string(), ImportSymbol::Commonjs);
      }
      Callee::Super(_) => {}
    }
  }
}
