use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

use once_cell::sync::Lazy;
use swc_core::atoms::Atom;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Fold;
use swc_core::ecma::visit::FoldWith;
use std::sync::Mutex;

use crate::kit::swc::lookup_property_access;
use crate::kit::swc::stmt_to_module_item;
use crate::kit::swc::PropAccessType;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::BundleGraph;
use crate::public::DependencyMap;

use super::super::runtime_factory::ExportNamed;
use super::super::runtime_factory::ImportNamed;
use super::super::runtime_factory::RuntimeFactory;

use super::read_exports::read_exports;
use super::read_exports_named::read_exports_named;
use super::read_exports_named::ExportAssignment;
use super::read_import_assignments::read_import_assignments;
use super::read_import_assignments::ImportAssignment;

static REQUIRE_SYMBOL: Lazy<Atom> = Lazy::new(|| Atom::from("require"));

pub struct JavaScriptRuntime<'a> {
  pub current_asset_id: &'a Path,
  pub current_bundle_id: &'a str,
  pub dependency_map: &'a DependencyMap,
  pub asset_graph: &'a AssetGraph,
  pub asset_map: Arc<Mutex<AssetMap>>,
  pub bundle_graph: &'a BundleGraph,
  pub runtime_factory: &'a RuntimeFactory,
  pub depends_on: HashSet<String>,
}

impl<'a> JavaScriptRuntime<'a> {
  fn get_bundle_ids_and_asset_id(
    &mut self,
    specifier: &str,
  ) -> Option<(Vec<String>, String)> {
    let Some(dependency) = self
      .dependency_map
      .get_dependency_for_specifier(&self.current_asset_id, specifier)
    else {
      panic!(
        "Could not get dependency for specifier:\n  Asset: {:?}\n  Specifier: {:?}",
        self.current_asset_id, specifier
      );
    };

    let Some(asset_id) = self.asset_graph.get_asset_id_for_dependency(&dependency) else {
      panic!(
        "Could not get asset_id for dependency:\n  Dependency: {:?}",
        dependency.id
      );
    };

    let asset_kind = {
      let asset_map = self.asset_map.lock().unwrap();
      let Some(asset) = asset_map.get(&asset_id) else {
        panic!(
          "Could not get Asset for AssetID:\n  AssetID: {:?}",
          asset_id
        );
      };
      asset.kind.clone()
    };

    if asset_kind != "js" {
      return None;
    }

    let Some(bundle_id) = self.bundle_graph.get(&dependency.id) else {
      panic!(
        "Could not get Bundle for Dependency:\n  Dependency: {}",
        dependency.id
      );
    };

    if bundle_id == self.current_bundle_id {
      return Some((vec![], asset_id.to_str().unwrap().to_string()));
    } else {
      self.depends_on.insert(bundle_id.clone());
      return Some((
        vec![bundle_id.clone()],
        asset_id.to_str().unwrap().to_string(),
      ));
    }
  }

  fn create_import_named(
    &mut self,
    specifier: &str,
    assignments: Vec<ImportNamed>,
  ) -> Option<Stmt> {
    let Some((bundle_ids, asset_id)) = self.get_bundle_ids_and_asset_id(specifier) else {
      return None;
    };
    return Some(
      self
        .runtime_factory
        .mach_require_named(assignments, &asset_id, &bundle_ids),
    );
  }

  fn create_import_namespace(
    &mut self,
    specifier: &str,
    assignment: Option<String>,
  ) -> Option<Stmt> {
    let Some((bundle_ids, asset_id)) = self.get_bundle_ids_and_asset_id(specifier) else {
      return None;
    };
    return Some(
      self
        .runtime_factory
        .mach_require_namespace(assignment, &asset_id, &bundle_ids),
    );
  }

  fn create_export_namespace(
    &mut self,
    specifier: &str,
    assignment: Option<String>,
  ) -> Option<Stmt> {
    let Some((bundle_ids, asset_id)) = self.get_bundle_ids_and_asset_id(specifier) else {
      return None;
    };
    return Some(self.runtime_factory.define_reexport_namespace(
      assignment,
      &asset_id,
      &bundle_ids,
    ));
  }
}

impl<'a> Fold for JavaScriptRuntime<'a> {
  fn fold_module(
    &mut self,
    module: Module,
  ) -> Module {
    let mut module = module.fold_children_with(self);

    let mut statements = Vec::<Stmt>::new();

    for decl in module.body.drain(0..) {
      match decl {
        ModuleItem::ModuleDecl(decl) => {
          match decl {
            ModuleDecl::Import(decl) => {
              let specifier = &decl.src.value.to_string();

              match read_import_assignments(&decl) {
                /*
                  import * as foo from './foo'
                */
                ImportAssignment::Star(name) => {
                  if let Some(stmt) = self.create_import_namespace(&specifier, Some(name)) {
                    statements.push(stmt);
                  }
                }
                /*
                  import a, { b } from './foo'
                  import foo './foo'
                */
                ImportAssignment::Named(names) => {
                  if let Some(stmt) = self.create_import_named(&specifier, names) {
                    statements.push(stmt);
                  }
                }
                /*
                  import './foo'
                */
                ImportAssignment::None => {
                  if let Some(stmt) = self.create_import_namespace(&specifier, None) {
                    statements.push(stmt);
                  }
                }
              }
            }
            /*
              export const foo = ''
              export function foo() {}
              export class foo {}
            */
            ModuleDecl::ExportDecl(decl) => {
              // TODO Don't use a getter if the value is never reassigned
              statements.push(Stmt::Decl(decl.decl.clone()));

              for export in read_exports(decl) {
                statements.push(
                  self
                    .runtime_factory
                    .define_export(&export, &export, true, false),
                );
              }
            }
            ModuleDecl::ExportNamed(decl) => {
              // let specifier = &decl.src.value.to_string();
              let mut specifier = None::<String>;

              if let Some(src) = &decl.src {
                specifier = Some(src.value.to_string());
              }

              match read_exports_named(decl, specifier) {
                /*
                  export { foo } from './foo'
                  export { foo as bar } from './foo'
                */
                ExportAssignment::ReexportNamed(reexports, specifier) => {
                  let Some((bundle_ids, module_id)) = self.get_bundle_ids_and_asset_id(&specifier)
                  else {
                    continue;
                  };

                  statements.push(self.runtime_factory.define_reexport_named(
                    &reexports,
                    &module_id,
                    &bundle_ids,
                  ));
                }
                /*
                  export * as foo from './foo'
                */
                ExportAssignment::ReexportNamespace(namespace, specifier) => {
                  if let Some(stmt) = self.create_export_namespace(&specifier, Some(namespace)) {
                    statements.push(stmt);
                  }
                }
                /*
                  const foo = ''; export { foo }
                  const foo = ''; export { foo as bar }
                */
                ExportAssignment::ExportNamed(assignments) => {
                  for assignment in assignments {
                    match assignment {
                      ExportNamed::Named(key) => {
                        statements
                          .push(self.runtime_factory.define_export(&key, &key, true, false));
                      }
                      ExportNamed::Renamed(key, key_as) => {
                        statements.push(
                          self
                            .runtime_factory
                            .define_export(&key_as, &key, true, false),
                        );
                      }
                      ExportNamed::Default(_) => panic!("impossible"),
                    }
                  }
                }
              }
            }
            ModuleDecl::ExportDefaultDecl(decl) => {
              match decl.decl {
                /*
                  export default class foo {}
                  export default class {}
                */
                DefaultDecl::Class(decl) => {
                  if let Some(ident) = decl.ident {
                    let class_name = ident.sym.to_string();
                    let stmt = Stmt::Decl(Decl::Class(ClassDecl {
                      ident,
                      declare: false,
                      class: decl.class,
                    }));
                    statements.push(stmt);
                    statements.push(
                      self
                        .runtime_factory
                        .define_export_default_named(&class_name),
                    );
                  } else {
                    statements.push(self.runtime_factory.define_export_default(Expr::Class(
                      ClassExpr {
                        ident: None,
                        class: decl.class,
                      },
                    )));
                  }
                }
                /*
                  export default function foo() {}
                  export default function() {}
                */
                DefaultDecl::Fn(decl) => {
                  if let Some(ident) = decl.ident {
                    let func_name = ident.sym.to_string();
                    let stmt = Stmt::Decl(Decl::Fn(FnDecl {
                      ident,
                      declare: false,
                      function: decl.function,
                    }));
                    statements.push(stmt);
                    statements.push(self.runtime_factory.define_export_default_named(&func_name));
                  } else {
                    statements.push(self.runtime_factory.define_export_default(Expr::Fn(FnExpr {
                      ident: None,
                      function: decl.function,
                    })));
                  }
                }
                _ => panic!("Not implemented"),
              }
            }
            /*
              export default 42
              export default ''
            */
            ModuleDecl::ExportDefaultExpr(decl) => {
              statements.push(self.runtime_factory.define_export_default(*decl.expr));
            }
            /*
              export * from './foo'
            */
            ModuleDecl::ExportAll(decl) => {
              let specifier = &decl.src.value.to_string();
              if let Some(stmt) = self.create_export_namespace(&specifier, None) {
                statements.push(stmt);
              }
            }
            _ => panic!("not implemented"),
          }
        }
        ModuleItem::Stmt(stmt) => {
          statements.push(stmt);
        }
      }
    }

    module.body = stmt_to_module_item(statements);

    return module;
  }

  fn fold_call_expr(
    &mut self,
    expr: CallExpr,
  ) -> CallExpr {
    let call_expr = expr.fold_children_with(self);

    match &call_expr.callee {
      Callee::Expr(callee_expr) => {
        let Expr::Ident(ident) = &**callee_expr else {
          return call_expr;
        };
        if ident.sym != *REQUIRE_SYMBOL {
          return call_expr;
        }
        let Expr::Lit(import_specifier_arg) = &*call_expr.args[0].expr else {
          return call_expr;
        };
        let Lit::Str(import_specifier) = import_specifier_arg else {
          return call_expr;
        };

        let Some((bundle_ids, asset_id)) =
          self.get_bundle_ids_and_asset_id(&import_specifier.value.to_string())
        else {
          return call_expr;
        };

        let mach_require = self
          .runtime_factory
          .mach_require(&asset_id, &bundle_ids, None)
          .as_expr()
          .unwrap()
          .to_owned()
          .expr;

        let Expr::Call(result) = *mach_require else {
          panic!()
        };
        return result;
      }
      Callee::Import(_) => {
        let Expr::Lit(import_specifier_arg) = &*call_expr.args[0].expr else {
          return call_expr;
        };
        let Lit::Str(import_specifier) = import_specifier_arg else {
          return call_expr;
        };

        let Some((bundle_ids, asset_id)) =
          self.get_bundle_ids_and_asset_id(&import_specifier.value)
        else {
          return call_expr;
        };

        let import_stmt = self
          .runtime_factory
          .mach_require(&asset_id, &bundle_ids, None);

        let Stmt::Expr(import_stmt) = import_stmt else {
          panic!("");
        };

        let Expr::Await(import_stmt) = *import_stmt.expr else {
          panic!()
        };

        let Expr::Call(result) = *import_stmt.arg else {
          panic!()
        };

        return result;
      }
      Callee::Super(_) => {}
    };

    return call_expr;
  }

  /*
    module.exports.a
    module.export
    exports.a
  */
  fn fold_member_expr(
    &mut self,
    member_expression: MemberExpr,
  ) -> MemberExpr {
    let member_expression = member_expression.fold_children_with(self);

    let Ok(prop_assignment) = ('block: {
      if let Ok(prop) = lookup_property_access(&member_expression, &["module", "exports"]) {
        break 'block Ok(prop);
      };
      if let Ok(prop) = lookup_property_access(&member_expression, &["exports"]) {
        break 'block Ok(prop);
      };
      break 'block Err(());
    }) else {
      return member_expression;
    };

    if let Some(key) = prop_assignment {
      match key {
        PropAccessType::Ident(_, key) => {
          let key = self.runtime_factory.create_string(&key);
          let result = self.runtime_factory.module_exports_access(Some(key));
          let Stmt::Expr(result) = result else { panic!() };
          let Expr::Member(result) = *result.expr else {
            panic!()
          };
          return result;
        }
        PropAccessType::Computed(expr) => {
          let result = self.runtime_factory.module_exports_access(Some(expr));
          let Stmt::Expr(result) = result else { panic!() };
          let Expr::Member(result) = *result.expr else {
            panic!()
          };
          return result;
        }
      }
    }
    let result = self.runtime_factory.module_exports_access(None);
    let Stmt::Expr(result) = result else { panic!() };
    let Expr::Member(result) = *result.expr else {
      panic!()
    };
    return result;
  }

  /*
    module.exports.a = value
    module.exports = value
    exports.a = value
  */
  fn fold_assign_expr(
    &mut self,
    assign: AssignExpr,
  ) -> AssignExpr {
    let mut assign = assign.fold_children_with(self);

    let PatOrExpr::Pat(pat) = &assign.left else {
      return assign;
    };
    let Pat::Expr(expr) = &**pat else {
      return assign;
    };
    let Expr::Member(member_expression) = &**expr else {
      return assign;
    };

    let Ok(prop_assignment) = ('block: {
      if let Ok(prop) = lookup_property_access(&member_expression, &["module", "exports"]) {
        break 'block Ok(prop);
      };
      if let Ok(prop) = lookup_property_access(&member_expression, &["exports"]) {
        break 'block Ok(prop);
      };
      break 'block Err(());
    }) else {
      return assign;
    };

    if let Some(key) = prop_assignment {
      match key {
        PropAccessType::Ident(_, key) => {
          let key = self.runtime_factory.create_string(&key);
          let result = self
            .runtime_factory
            .module_exports_assign(Some(key), *assign.right);
          let Stmt::Expr(result) = result else { panic!() };
          let Expr::Assign(result) = *result.expr else {
            panic!()
          };
          return result;
        }
        PropAccessType::Computed(expr) => {
          let result = self
            .runtime_factory
            .module_exports_assign(Some(expr), *assign.right);
          let Stmt::Expr(result) = result else { panic!() };
          let Expr::Assign(result) = *result.expr else {
            panic!()
          };
          return result;
        }
      }
    }

    if let Expr::Call(call) = *assign.right {
      assign.right = Box::new(Expr::Call(self.fold_call_expr(call)));
    }

    let result = self
      .runtime_factory
      .module_exports_assign(None, *assign.right);
    let Stmt::Expr(result) = result else { panic!() };
    let Expr::Assign(result) = *result.expr else {
      panic!()
    };
    return result;
  }
}
