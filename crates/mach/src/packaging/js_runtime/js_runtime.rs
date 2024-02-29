use std::collections::HashSet;
use std::path::Path;

use once_cell::sync::Lazy;
use swc_core::atoms::Atom;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Fold;
use swc_core::ecma::visit::FoldWith;

use crate::packaging::runtime_factory::ExportNamed;
use crate::packaging::runtime_factory::ImportNamed;
use crate::platform::swc::stmt_to_module_item;
use crate::public::AssetGraph;
use crate::public::BundleGraph;
use crate::public::DependencyMap;

use super::super::runtime_factory::RuntimeFactory;
use super::read_exports::read_exports;
use super::read_exports_named::read_exports_named;
use super::read_exports_named::ExportAssignment;
use super::read_import_assignments::read_import_assignments;
use super::read_import_assignments::ImportAssignment;

static REQUIRE_SYMBOL: Lazy<Atom> = Lazy::new(|| Atom::from("require"));

#[derive(Debug, Clone)]
pub enum ModuleKind {
  Unknown,
  ESM,
  Commonjs,
}

pub struct JavaScriptRuntime<'a> {
  pub depends_on_bundles: HashSet<String>,
  pub current_asset_id: &'a Path,
  pub current_bundle_id: &'a str,
  pub dependency_map: &'a DependencyMap,
  pub asset_graph: &'a AssetGraph,
  pub bundle_graph: &'a BundleGraph,
  pub runtime_factory: &'a RuntimeFactory,
}

impl<'a> JavaScriptRuntime<'a> {
  fn add_bundle_dependency(&mut self, bundle_ids: &[String]) {
    for bundle_id in bundle_ids {
      if bundle_id != self.current_bundle_id {
        self.depends_on_bundles.insert(bundle_id.to_string());
      }
    }
  }

  fn get_bundle_ids_and_asset_id(
    &self,
    specifier: &str,
  ) -> (Vec<String>, String) {
    let (dependency_id, dependency) = 'block: {
      for (dependency_id, dependency) in &self.dependency_map.dependencies {
        if dependency.specifier == *specifier {
          break 'block (dependency_id, dependency);
        }
      }
      panic!(
        "Could not find dependency for specifier\n  {}\n  {:?}",
        specifier, self.current_asset_id
      );
    };

    let asset_graph_entries = self
      .asset_graph
      .get_dependencies(&dependency.resolve_from_rel)
      .unwrap();
    let mut asset_id = "";
    for (dep_id, target_asset_id) in asset_graph_entries {
      if dep_id == dependency_id {
        asset_id = target_asset_id.to_str().unwrap();
      }
    }

    let bundle_id = self.bundle_graph.get(dependency_id).unwrap();
    if bundle_id == self.current_bundle_id {
      return (vec![], asset_id.to_string());
    } else {
      return (vec![bundle_id.clone()], asset_id.to_string());
    }
  }

  fn create_import_named(
    &self,
    specifier: &str,
    assignments: Vec<ImportNamed>,
  ) -> (Stmt, Vec<String>) {
    let (bundle_ids, asset_id) = self.get_bundle_ids_and_asset_id(specifier);
    return (
      self.runtime_factory.mach_require_named(
      assignments,
      &asset_id,
      &bundle_ids,
    ), bundle_ids);
  }

  fn create_import_namespace(
    &self,
    specifier: &str,
    assignment: Option<String>,
  ) -> (Stmt, Vec<String>) {
    let (bundle_ids, asset_id) = self.get_bundle_ids_and_asset_id(specifier);
    return (self.runtime_factory.mach_require_namespace(
      assignment,
      &asset_id,
      &bundle_ids,
    ), bundle_ids);
  }

  fn create_export_namespace(
    &self,
    specifier: &str,
    assignment: Option<String>,
  ) -> (Stmt, Vec<String>) {
    let (bundle_ids, asset_id) = self.get_bundle_ids_and_asset_id(specifier);
    return (self.runtime_factory.define_reexport_namespace(
      assignment,
      &asset_id,
      &bundle_ids,
    ), bundle_ids);
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
                  let (stmt, bundle_ids) = self.create_import_namespace(&specifier, Some(name));
                  statements.push(stmt);
                  self.add_bundle_dependency(&bundle_ids);
                }
                /*
                  import a, { b } from './foo'
                  import foo './foo'
                */
                ImportAssignment::Named(names) => {
                  let (stmt, bundle_ids) = self.create_import_named(&specifier, names);
                  statements.push(stmt);
                  self.add_bundle_dependency(&bundle_ids);
                }
                /*
                  import './foo'
                */
                ImportAssignment::None => {
                  statements.push(self.create_import_namespace(&specifier, None));
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
                statements.push(self.runtime_factory.define_export(&export, &export));
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
                  let (bundle_ids, module_id) = self.get_bundle_ids_and_asset_id(&specifier);
      
                  statements.push(self.runtime_factory.define_reexport_named(&reexports, &module_id, &bundle_ids));
                }
                /*
                  export * as foo from './foo'
                */
                ExportAssignment::ReexportNamespace(namespace, specifier) => {
                  statements.push(self.create_export_namespace(&specifier, Some(namespace)));
                }
                /*
                  const foo = ''; export { foo }
                  const foo = ''; export { foo as bar }
                */
                ExportAssignment::ExportNamed(assignments) => {
                  for assignment in assignments {
                    match assignment {
                      ExportNamed::Named(key) => {
                        statements.push(self.runtime_factory.define_export(&key, &key));
                      }
                      ExportNamed::Renamed(key, key_as) => {
                        statements.push(self.runtime_factory.define_export(&key_as, &key));
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
                    statements.push(self.runtime_factory.define_export_default_named(&class_name));
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
              statements.push(self.create_export_namespace(&specifier, None))
            }
            _ => panic!("not implemented"),
          }
        }
        ModuleItem::Stmt(decl) => {
          statements.push(decl.clone());
        }
      }
    }

    module.body = stmt_to_module_item(statements);

    return module;
  }

  fn fold_call_expr(
    &mut self,
    call_expr: CallExpr,
  ) -> CallExpr {
    let call_expr = call_expr.fold_children_with(self);

    match &call_expr.callee {
      /*
         import("specifier")
      */
      Callee::Import(_) => {
        if call_expr.args.len() == 0 {
          return call_expr;
        }
        let Expr::Lit(import_specifier_arg) = &*call_expr.args[0].expr else {
          return call_expr;
        };
        let Lit::Str(import_specifier) = import_specifier_arg else {
          return call_expr;
        };
        let (bundle_ids, asset_id) = self.get_bundle_ids_and_asset_id(&import_specifier.value);

        for bundle_id in &bundle_ids {
          self.depends_on_bundles.insert(bundle_id.to_string());
        }

        let import_stmt = self
          .runtime_factory
          .mach_require(&asset_id, &bundle_ids);

        let Stmt::Expr(import_stmt) = import_stmt else {
          panic!("Unable to generate import");
        };

        let Expr::Call(import_stmt) = *import_stmt.expr else {
          panic!("Unable to generate import");
        };

        return import_stmt;
      }

      Callee::Expr(_) => {}
      Callee::Super(_) => {}
    };

    return call_expr;
  }

  /*
    require("specifier")
  */
  fn fold_expr(
    &mut self,
    expr: Expr,
  ) -> Expr {
    let expr = expr.fold_children_with(self);
    if let Expr::Call(call_expr) = expr {
      match &call_expr.callee {
        Callee::Expr(expr) => {
          let Expr::Ident(ident) = &**expr else {
            return Expr::Call(call_expr);
          };
          if ident.sym != *REQUIRE_SYMBOL {
            return Expr::Call(call_expr);
          }
          let Expr::Lit(import_specifier_arg) = &*call_expr.args[0].expr else {
            return Expr::Call(call_expr);
          };
          let Lit::Str(import_specifier) = import_specifier_arg else {
            return Expr::Call(call_expr);
          };

          let (bundle_ids, asset_id) = self.get_bundle_ids_and_asset_id(&import_specifier.value.to_string());
          self.add_bundle_dependency(&bundle_ids);
  
          let mach_require = self
            .runtime_factory
            .mach_require(&asset_id, &bundle_ids)
            .as_expr().unwrap().to_owned()
            .expr;

          return *mach_require;
        }
        Callee::Import(_) => {}
        Callee::Super(_) => {}
      };
      return Expr::Call(call_expr);
    };

    return expr;
  }
}
