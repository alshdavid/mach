use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Fold;
use swc_core::ecma::visit::FoldWith;

use crate::public::BundleGraph;
use crate::public::DependencyMap;

use super::runtime_factory::RuntimeFactory;

pub struct JavaScriptRuntime<'a> {
  pub dependency_map: &'a DependencyMap,
  pub bundle_graph: &'a BundleGraph,
  pub runtime_factory: &'a RuntimeFactory,
}

impl<'a> Fold for JavaScriptRuntime<'a> {
  fn fold_module(
    &mut self,
    module: Module,
  ) -> Module {
    let mut module = module.fold_children_with(self);

    let mut statements = Vec::<Stmt>::new();

    for i in 0..module.body.len() {
      match &module.body[i] {
        ModuleItem::ModuleDecl(decl) => {
          match decl {
            ModuleDecl::Import(decl) => {
              let specifier = &decl.src.value.to_string();

              let (dependency_id, dependency) = 'block: {
                for (dependency_id, dependency) in &self.dependency_map.dependencies {
                  if dependency.specifier == *specifier {
                    break 'block (dependency_id, dependency);
                  }
                }
                panic!();
              };

              let bundle_id = self.bundle_graph.get(dependency_id).unwrap();
              statements.push(self.runtime_factory.require_async(&[bundle_id], &specifier));
            },
            ModuleDecl::ExportDecl(decl) => {

              // dbg!(&decl);
            },
            ModuleDecl::ExportNamed(decl) => {
              // dbg!(&decl);
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
          // dbg!(&decl);
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
}