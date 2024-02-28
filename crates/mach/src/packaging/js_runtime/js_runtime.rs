use std::path::Path;

use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Fold;
use swc_core::ecma::visit::FoldWith;

use crate::packaging::runtime_factory::ExportNamed;
use crate::packaging::runtime_factory::ImportNamed;
use crate::public::AssetGraph;
use crate::public::BundleGraph;
use crate::public::DependencyMap;

use super::super::runtime_factory::RuntimeFactory;
use super::read_exports::read_exports;
use super::read_exports_named::read_exports_named;
use super::read_exports_named::ExportAssignment;
use super::read_import_assignments::read_import_assignments;

pub struct JavaScriptRuntime<'a> {
  pub current_asset_id: &'a Path,
  pub current_bundle_id: &'a str,
  pub dependency_map: &'a DependencyMap,
  pub asset_graph: &'a AssetGraph,
  pub bundle_graph: &'a BundleGraph,
  pub runtime_factory: &'a RuntimeFactory,
}

impl<'a> JavaScriptRuntime<'a> {
  fn get_bundle_ids_and_asset_id(
    &self,
    specifier: &str,
  ) -> (Vec<&str>, String) {
    let (dependency_id, dependency) = 'block: {
      for (dependency_id, dependency) in &self.dependency_map.dependencies {
        if dependency.specifier == *specifier {
          break 'block (dependency_id, dependency);
        }
      }
      panic!();
    };

    let asset_graph_entries = self.asset_graph.get_dependencies(&dependency.resolve_from_rel).unwrap();
    let mut asset_id = "";
    for (dep_id, target_asset_id) in asset_graph_entries {
      if dep_id == dependency_id {
        asset_id = target_asset_id.to_str().unwrap();
      }
    }

    let bundle_id = self.bundle_graph.get(dependency_id).unwrap();
    if bundle_id == self.current_bundle_id {
      return (vec![bundle_id], asset_id.to_string());
    } else {
      return (vec![bundle_id], asset_id.to_string());
    }
  }

  /// Create "const { foo } = await mach_import([bundle_ids], module_id)""
  fn create_import_named(
    &self, 
    specifier: &str,
    assignments: Vec<ImportNamed>,
  ) -> Stmt {
    let (bundle_ids, asset_id) = self.get_bundle_ids_and_asset_id(specifier);
    return self.runtime_factory.require_async_named(&bundle_ids.as_slice(), &asset_id, assignments);
  }

  /// Create "const foo = await mach_import([bundle_ids], module_id)""
  fn create_import_namespace(
    &self, 
    specifier: &str,
    assignment: Option<String>,
  ) -> Stmt {
    let (bundle_ids, asset_id) = self.get_bundle_ids_and_asset_id(specifier);
    return self.runtime_factory.require_async_namespace(&bundle_ids.as_slice(), &asset_id, assignment);
  }
}

impl<'a> Fold for JavaScriptRuntime<'a> {
  fn fold_module(
    &mut self,
    module: Module,
  ) -> Module {
    let mut module = module.fold_children_with(self);

    let mut statements = Vec::<Stmt>::new();

    while let Some(decl) = module.body.pop() { 
      match decl {
        ModuleItem::ModuleDecl(decl) => {
          match decl {
            ModuleDecl::Import(decl) => {
              let specifier = &decl.src.value.to_string();

              match read_import_assignments(&decl) {
                /*
                  import * as foo from './foo'
                */
                super::read_import_assignments::ImportAssignment::Star(name) => {
                  statements.push(self.create_import_namespace(&specifier, Some(name)));
                },
                /*
                  import a, { b } from './foo'
                  import foo './foo'
                */
                super::read_import_assignments::ImportAssignment::Named(names) => {
                  statements.push(self.create_import_named(&specifier, names));
                },
                 /*
                  import './foo'
                */
                super::read_import_assignments::ImportAssignment::None => {
                  statements.push(self.create_import_namespace(&specifier, None));
                },
              }
            },
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
            },
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
                  let mut reexports_stmts = Vec::<Stmt>::new();
                  reexports_stmts.push(self.create_import_named(&specifier, reexports.clone()));

                  for reexport in reexports {
                    match reexport {
                      ExportNamed::Named(key) => reexports_stmts.push(
                        self.runtime_factory.define_export(&key, &key)
                      ),
                      ExportNamed::Renamed(key, key_as) => reexports_stmts.push(
                        self.runtime_factory.define_export(&key_as, &key)
                      ),
                      ExportNamed::Default(_) => panic!("impossible"),
                    }
                  }
                  statements.push(self.runtime_factory.wrapper(reexports_stmts));
                },
                /*
                  export * as foo from './foo'
                */
                ExportAssignment::ReexportNamespace(_, _) => todo!(),
                /*
                  const foo = ''; export { foo }
                  const foo = ''; export { foo as bar }
                */
                ExportAssignment::ExportNamed(assignments) => {
                  for assignment in assignments {
                    match assignment {
                        ExportNamed::Named(key) => {
                          statements.push(self.runtime_factory.define_export(&key, &key));
                        },
                        ExportNamed::Renamed(key, key_as) => {
                          statements.push(self.runtime_factory.define_export(&key_as, &key));
                        },
                        ExportNamed::Default(_) => panic!("impossible"),
                    }
                  }
                },
              }
            },
            ModuleDecl::ExportDefaultDecl(decl) => {
              // dbg!(&decl);
            },
            ModuleDecl::ExportDefaultExpr(decl) => {
              // dbg!(&decl);
            },
            ModuleDecl::ExportAll(decl) => {
              // dbg!(&decl);
            },
            ModuleDecl::TsImportEquals(decl) => {
              // dbg!(&decl);
            },
            ModuleDecl::TsExportAssignment(decl) => {
              // dbg!(&decl);
            },
            ModuleDecl::TsNamespaceExport(decl) => {
              // dbg!(&decl);
            },
          }
        },
        ModuleItem::Stmt(decl) => {
          statements.push(decl.clone());
        },
      }
    }

    module.body = vec![];

    for stmt in statements {
      module.body.push(ModuleItem::Stmt(stmt));
    }

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

        let import_stmt = self.runtime_factory.require_async(&bundle_ids.as_slice(), &asset_id);

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
}