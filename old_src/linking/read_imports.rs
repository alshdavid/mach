use once_cell::sync::Lazy;
use swc_core::atoms::Atom;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Visit;
use swc_core::ecma::visit::VisitWith;

use crate::public::DependencyKind;

static REQUIRE_SYMBOL: Lazy<Atom> = Lazy::new(|| Atom::from("require"));

#[derive(Debug)]
pub struct ImportReadResult {
  pub specifier: String,
  pub kind: DependencyKind
}

pub fn read_imports(module: &Module) -> Vec<ImportReadResult> {
  let mut w = Walker {
    imports: Vec::<ImportReadResult>::new(),
  };
  module.visit_with(&mut w);
  return w.imports;
}

struct Walker {
  imports: Vec<ImportReadResult>,
}

impl Visit for Walker {
  // Recursive
  fn visit_module(&mut self, n: &Module) {
    n.visit_children_with(self);
  }

  // import "specifier"
  // import {} from "specifier"
  // import * as foo from "specifier"
  // import foo from "specifier"
  fn visit_import_decl(&mut self, node: &ImportDecl) {
    if node.type_only {
      return;
    }
    self
      .imports
      .push(ImportReadResult {
        specifier: node.src.value.to_string(),
        kind: DependencyKind::Static,
      })
  }

  // export * as foo from "specifier"
  fn visit_export_all(&mut self, node: &ExportAll) {
    if node.type_only {
      return;
    }
    self
      .imports
      .push(ImportReadResult {
        specifier: node.src.value.to_string(),
        kind: DependencyKind::Static,
      })
  }

  // export {} from "specifier"
  fn visit_named_export(&mut self, node: &NamedExport) {
    if node.type_only {
      return;
    }
    if let Some(src) = &node.src {
      self
        .imports
        .push(ImportReadResult {
          specifier: src.value.to_string(),
          kind: DependencyKind::Static,
        })
    }
  }

  // import("specifier")
  // require("specifier")
  fn visit_call_expr(&mut self, node: &CallExpr) {
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
        self.imports.push(ImportReadResult {
          specifier: import_specifier.value.to_string(),
          kind: DependencyKind::Dynamic,
        });
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
        self.imports.push(ImportReadResult {
          specifier: import_specifier.value.to_string(),
          kind: DependencyKind::Require,
        });
      }
      Callee::Super(_) => {}
    }
  }
}
