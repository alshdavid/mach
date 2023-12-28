use std::sync::Arc;

use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Fold;
use swc_core::ecma::visit::FoldWith;

use crate::package::runtime::RuntimeFactory;

use super::read_exports;
use super::read_exports_named;
use super::read_import_assignments;
use super::ExportAssignment;
use super::ExportNamedAssignment;
use super::ImportAssignment;
use crate::bundle::BundleDependencyIndex;

pub struct ApplyRuntimeEsm {
  pub asset_id: String,
  pub dependency_index: Arc<BundleDependencyIndex>,
  pub runtime_factory: Arc<RuntimeFactory>,
}

impl Fold for ApplyRuntimeEsm {
  fn fold_module(&mut self, module: Module) -> Module {
    let mut module = module.fold_children_with(self);

    let mut statements = Vec::<Stmt>::new();

    for i in 0..module.body.len() {
      match &module.body[i] {
        ModuleItem::ModuleDecl(decl) => {
          match decl {
            ModuleDecl::Import(import_decl) => {
              let specifier = &import_decl.src.value.to_string();
              let (asset_id, _) = self
                .dependency_index
                .get(&(self.asset_id.to_string(), specifier.clone()))
                .unwrap();

              match read_import_assignments(&import_decl) {
                /*
                  import a, { b } from './foo'
                  import foo './foo'
                */
                ImportAssignment::Named(assignments) => {
                  statements.push(self.runtime_factory.import_named(&asset_id, &assignments));
                }
                /*
                  import * as foo from './foo'
                */
                ImportAssignment::Star(assignment) => {
                  statements.push(self.runtime_factory.import_star(&asset_id, &assignment));
                }
                /*
                  import './foo'
                */
                ImportAssignment::None => {
                  statements.push(self.runtime_factory.import(&asset_id));
                }
              }
            }
            /*
              export const foo = ''
              export function foo() {}
              export class foo {}
            */
            ModuleDecl::ExportDecl(decl) => {
              statements.push(Stmt::Decl(decl.decl.clone()));

              for export in read_exports(&decl) {
                statements.push(self.runtime_factory.export(&export));
              }
            }
            ModuleDecl::ExportNamed(decl) => {
              match read_exports_named(decl, &self.asset_id, &self.dependency_index) {
                /*
                  export { foo } from './foo'
                  export { foo as bar } from './foo'
                */
                ExportAssignment::ReexportNamed(asset_id, exports) => {
                  for export in exports {
                    match export {
                      ExportNamedAssignment::NamedKey(export) => {
                        statements.push(self.runtime_factory.reexport(&asset_id, &export));
                      }
                      ExportNamedAssignment::RenamedKey(export, exported_as) => {
                        statements.push(self.runtime_factory.reexport_rename(
                          &asset_id,
                          &export,
                          &exported_as,
                        ));
                      }
                    }
                  }
                }
                /*
                  export * as foo from './foo'
                */
                ExportAssignment::ReexportNamespace(asset_id, export_as) => {
                  statements.push(
                    self
                      .runtime_factory
                      .reexport_all_rename(&asset_id, &export_as),
                  );
                }
                /*
                  const foo = ''; export { foo }
                  const foo = ''; export { foo as bar }
                */
                ExportAssignment::ExportNamed(exports) => {
                  for export in exports {
                    match export {
                      ExportNamedAssignment::NamedKey(export) => {
                        statements.push(self.runtime_factory.export(&export));
                      }
                      ExportNamedAssignment::RenamedKey(export, exported_as) => {
                        statements.push(self.runtime_factory.export_renamed(&export, &exported_as));
                      }
                    }
                  }
                }
              }
            }
            ModuleDecl::ExportDefaultDecl(decl) => {
              match &decl.decl {
                /*
                  export default class foo {}
                  export default class {}
                */
                DefaultDecl::Class(decl) => {
                  // If the class has a name
                  if let Some(ident) = &decl.ident {
                    let stmt = Stmt::Decl(Decl::Class(ClassDecl {
                      ident: ident.clone(),
                      declare: false,
                      class: decl.class.clone(),
                    }));
                    statements.push(stmt);
                    statements.push(self.runtime_factory.export_default(&ident.sym.to_string()));
                  } else {
                    statements.push(
                      self
                        .runtime_factory
                        .export_default_expr(Box::new(Expr::Class(decl.clone()))),
                    );
                  }
                }
                /*
                  export default function foo() {}
                  export default function() {}
                */
                DefaultDecl::Fn(decl) => {
                  // If the function has a name
                  if let Some(ident) = &decl.ident {
                    let stmt = Stmt::Decl(Decl::Fn(FnDecl {
                      ident: ident.clone(),
                      declare: false,
                      function: decl.function.clone(),
                    }));
                    statements.push(stmt);
                    statements.push(self.runtime_factory.export_default(&ident.sym.to_string()));
                  } else {
                    statements.push(
                      self
                        .runtime_factory
                        .export_default_expr(Box::new(Expr::Fn(decl.clone()))),
                    );
                  }
                }
                _ => {
                  panic!("Not Implemented")
                }
              }
            }
            ModuleDecl::ExportDefaultExpr(decl) => {
              statements.push(self.runtime_factory.export_default_expr(decl.expr.clone()));
            }
            ModuleDecl::ExportAll(decl) => {
              let specifier = &decl.src.value.to_string();
              let (asset_id, _) = self
                .dependency_index
                .get(&(self.asset_id.to_string(), specifier.clone()))
                .unwrap();
              statements.push(self.runtime_factory.reexport_all(&asset_id))
            }
            ModuleDecl::TsImportEquals(_) => todo!(),
            ModuleDecl::TsExportAssignment(_) => todo!(),
            ModuleDecl::TsNamespaceExport(_) => todo!(),
          }
        }
        ModuleItem::Stmt(stmt) => {
          statements.push(stmt.clone());
        }
      }
    }

    module.body = vec![];

    for stmt in statements {
      module.body.push(ModuleItem::Stmt(stmt));
    }

    return module;
  }

  fn fold_call_expr(&mut self, call_expr: CallExpr) -> CallExpr {
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
        let (asset_id, _) = self
          .dependency_index
          .get(&(
            self.asset_id.to_string(),
            import_specifier.value.to_string(),
          ))
          .unwrap();
        return self.runtime_factory.import_dynamic_call_expr(&asset_id);
      }
      Callee::Expr(_) => {}
      Callee::Super(_) => {}
    };

    return call_expr;
  }
}
