use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Visit;
use swc_core::ecma::visit::VisitWith;

use crate::types::LinkingSymbol;

pub fn analyze_js_file_esm(module: &Program) -> Vec<LinkingSymbol> {
  let mut w = WalkerEsm { result: vec![] };

  module.visit_with(&mut w);

  return w.result;
}

#[derive(Debug)]
pub struct WalkerEsm {
  result: Vec<LinkingSymbol>,
}

impl Visit for WalkerEsm {
  fn visit_module(
    &mut self,
    module: &Module,
  ) {
    module.visit_children_with(self);

    'module_loop: for i in 0..module.body.len() {
      match &module.body[i] {
        ModuleItem::ModuleDecl(decl) => {
          match &decl {
            ModuleDecl::Import(decl) => {
              if decl.type_only {
                continue 'module_loop;
              }
              let import_specifier = decl.src.value.to_string();

              //
              // import './foo'
              //
              if decl.specifiers.len() == 0 {
                self.result.push(LinkingSymbol::ImportDirect {
                  specifier: import_specifier,
                });
                continue 'module_loop;
              }

              for specifier in &decl.specifiers {
                match &specifier {
                  // import { foo } from './foo'
                  // import { foo as bar } from './foo'
                  ImportSpecifier::Named(name) => {
                    //
                    // import { foo as bar } from './foo'
                    //
                    if let Some(imported) = &name.imported {
                      let ModuleExportName::Ident(imported) = imported else {
                        unreachable!();
                      };
                      self.result.push(LinkingSymbol::ImportRenamed {
                        sym: imported.sym.to_string(),
                        sym_as: name.local.sym.to_string(),
                        specifier: import_specifier.clone(),
                      });
                    }
                    //
                    // import { foo } from './foo'
                    //
                    else {
                      self.result.push(LinkingSymbol::ImportNamed {
                        sym: name.local.sym.to_string(),
                        specifier: import_specifier.clone(),
                      });
                    }
                  }
                  //
                  // import foo from './foo'
                  //
                  ImportSpecifier::Default(ident) => {
                    self.result.push(LinkingSymbol::ImportDefault {
                      sym_as: ident.local.sym.to_string(),
                      specifier: import_specifier.clone(),
                    });
                  }
                  //
                  // import * as foo from './foo'
                  //
                  ImportSpecifier::Namespace(decl) => {
                    self.result.push(LinkingSymbol::ImportNamespace {
                      sym_as: decl.local.sym.to_string(),
                      specifier: import_specifier.clone(),
                    });
                  }
                }
              }
            }

            ModuleDecl::ExportDecl(decl) => {
              match &decl.decl {
                //
                // export class foo {}
                //
                Decl::Class(decl) => {
                  self.result.push(LinkingSymbol::ExportNamed {
                    sym: decl.ident.sym.to_string(),
                  });
                }
                //
                // export function foo() {}
                //
                Decl::Fn(decl) => {
                  self.result.push(LinkingSymbol::ExportNamed {
                    sym: decl.ident.sym.to_string(),
                  });
                }

                Decl::Var(decl) => {
                  for var_decl in decl.decls.iter() {
                    match &var_decl.name {
                      //
                      // export const foo = ''
                      //
                      Pat::Ident(decl) => {
                        self.result.push(LinkingSymbol::ExportNamed {
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
                              self.result.push(LinkingSymbol::ExportDestructured {
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
                              self.result.push(LinkingSymbol::ExportDestructuredRenamed {
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
                              self.result.push(LinkingSymbol::ExportDestructured {
                                sym: ident.sym.to_string(),
                                sym_source: sym_source.sym.to_string(),
                              });
                            }
                            // TODO
                            //    export const [{ foo }] = obj
                            //    export const [[{ foo }]] = obj
                            //    export const [[ foo ]] = obj
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

            ModuleDecl::ExportNamed(decl) => {
              if decl.type_only {
                continue 'module_loop;
              }

              for specifier in &decl.specifiers {
                match &specifier {
                  //
                  // export * as foo from "specifier"
                  //
                  ExportSpecifier::Namespace(name) => {
                    match &name.name {
                      ModuleExportName::Ident(ident) => {
                        let Some(import_specifier) = &decl.src else {
                          continue 'module_loop;
                        };
                        self.result.push(LinkingSymbol::ReexportNamespace {
                          sym_as: ident.sym.to_string(),
                          specifier: import_specifier.value.to_string(),
                        })
                      }
                      ModuleExportName::Str(_) => todo!(),
                    };
                  }
                  //
                  // I don't think this can happen
                  //
                  ExportSpecifier::Default(_) => {
                    unreachable!()
                  }
                  ExportSpecifier::Named(name) => match &name.orig {
                    ModuleExportName::Ident(ident) => {
                      if let Some(import_specifier) = &decl.src {
                        let import_specifier = import_specifier.value.to_string();
                        //
                        // export { foo as bar } from "specifier"
                        //
                        if let Some(ModuleExportName::Ident(exported)) = &name.exported {
                          self.result.push(LinkingSymbol::ReexportRenamed {
                            sym: ident.sym.to_string(),
                            sym_as: exported.sym.to_string(),
                            specifier: import_specifier,
                          });
                        }
                        //
                        // export { foo } from "specifier"
                        //
                        else {
                          self.result.push(LinkingSymbol::ReexportNamed {
                            sym: ident.sym.to_string(),
                            specifier: import_specifier,
                          });
                        }
                      } else {
                        //
                        // export { foo as bar }"
                        //
                        if let Some(ModuleExportName::Ident(exported)) = &name.exported {
                          self.result.push(LinkingSymbol::ExportRenamed {
                            sym: ident.sym.to_string(),
                            sym_as: exported.sym.to_string(),
                          });
                        }
                        //
                        // export { foo }"
                        //
                        else {
                          self.result.push(LinkingSymbol::ExportNamed {
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

            //
            // export default class foo {}
            // export default class {}
            // export default function foo() {}
            // export default function() {}
            // export default ''
            //
            ModuleDecl::ExportDefaultDecl(_) => {
              self.result.push(LinkingSymbol::ExportDefault);
            }

            //
            // const foo = ''
            // export default foo
            //
            ModuleDecl::ExportDefaultExpr(_) => {
              self.result.push(LinkingSymbol::ExportDefault);
            }

            //
            // export * from "specifier"
            //
            ModuleDecl::ExportAll(decl) => {
              if decl.type_only {
                continue 'module_loop;
              }
              let import_specifier = decl.src.value.to_string();
              self.result.push(LinkingSymbol::ReexportAll {
                specifier: import_specifier,
              });
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

  fn visit_decl(
    &mut self,
    decl: &Decl,
  ) {
    match &decl {
      Decl::Var(var) => {
        for decl in &var.decls {
          // Check that this is an import() statement
          let Some(init) = &decl.init else {
            return;
          };
          let specifier = match &**init {
            // const foo = import()
            Expr::Call(call) => match call.callee {
              Callee::Import(_) => {
                let specifier_arg = call.args.get(0).unwrap();
                match &*specifier_arg.expr {
                  Expr::Lit(Lit::Str(lit_str)) => Some(lit_str.value.to_string()),
                  _ => None,
                }
              }
              _ => None,
            },
            // const foo = await import()
            Expr::Await(await_expr) => match &*await_expr.arg {
              Expr::Call(call) => match call.callee {
                Callee::Import(_) => {
                  let specifier_arg = call.args.get(0).unwrap();
                  match &*specifier_arg.expr {
                    Expr::Lit(Lit::Str(lit_str)) => Some(lit_str.value.to_string()),
                    _ => None,
                  }
                }
                _ => None,
              },
              _ => None,
            },
            _ => None,
          };
          let Some(specifier) = specifier else {
            return;
          };

          match &decl.name {
            // If it's a destructured object name the imports
            Pat::Object(object) => {
              for prop in &object.props {
                match prop {
                  // const { foo, bar } = import()
                  ObjectPatProp::Assign(assign) => {
                    self.result.push(LinkingSymbol::ImportDynamicNamed {
                      sym: assign.key.sym.to_string(),
                      specifier: specifier.clone(),
                    })
                  }
                  // const { foo: foo_renamed } = import()
                  ObjectPatProp::KeyValue(key_value) => {
                    println!("hiiii");
                    let PropName::Ident(key) = &key_value.key else {
                      self.result.push(LinkingSymbol::ImportDynamic { specifier });
                      return;
                    };
                    let Pat::Ident(value) = &*key_value.value else {
                      self.result.push(LinkingSymbol::ImportDynamic { specifier });
                      return;
                    };
                    self.result.push(LinkingSymbol::ImportDynamicRenamed {
                      sym: key.sym.to_string(),
                      sym_as: value.id.sym.to_string(),
                      specifier: specifier.clone(),
                    })
                  }
                  ObjectPatProp::Rest(_) => {
                    self.result.push(LinkingSymbol::ImportDynamic { specifier });
                    return;
                  }
                }
              }
            }
            // Otherwise we don't know what is being imported
            _ => self.result.push(LinkingSymbol::ImportDynamic { specifier }),
          }
        }
      }
      _ => {}
    };
  }

  fn visit_expr_stmt(
    &mut self,
    expr: &ExprStmt,
  ) {
    match &*expr.expr {
      Expr::Call(call) => {
        match &call.callee {
          //
          // import("specifier")
          //
          Callee::Import(_) => {
            if call.args.len() == 0 {
              return;
            }
            let Expr::Lit(import_specifier_arg) = &*call.args[0].expr else {
              return;
            };
            let Lit::Str(import_specifier) = import_specifier_arg else {
              return;
            };
            return self.result.push(LinkingSymbol::ImportDynamic {
              specifier: import_specifier.value.to_string(),
            });
          }
          _ => {}
        }
      }
      Expr::Await(await_expr) => match &*await_expr.arg {
        Expr::Call(call) => match call.callee {
          Callee::Import(_) => {
            let specifier_arg = call.args.get(0).unwrap();
            match &*specifier_arg.expr {
              Expr::Lit(Lit::Str(lit_str)) => {
                return self.result.push(LinkingSymbol::ImportDynamic {
                  specifier: lit_str.value.to_string(),
                });
              }
              _ => {}
            }
          }
          _ => {}
        },
        _ => {}
      },
      _ => {}
    }
  }
}
