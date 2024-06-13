use once_cell::sync::Lazy;
use swc_core::atoms::Atom;
use swc_core::common::Span;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Visit;
use swc_core::ecma::visit::VisitWith;
use crate::public::ModuleSymbol;


static REQUIRE_SYMBOL: Lazy<Atom> = Lazy::new(|| Atom::from("require"));

pub fn analyze_js_file_cjs(module: &Program) -> Vec<ModuleSymbol> {
  let mut w = WalkerCjs::default();

  module.visit_with(&mut w);

  if w.has_export {
    w.results.push(ModuleSymbol::ExportCommonjs);
  }

  return w.results;
}

#[derive(Debug, Default)]
pub struct WalkerCjs {
  has_export: bool,
  results: Vec<ModuleSymbol>,
}

impl WalkerCjs {
  fn module_exports(
    &mut self,
    member: &MemberExpr,
  ) {
    //
    // exports.a = ''
    // exports.a = foo
    //
    let Expr::Ident(first_ident) = &*member.obj else {
      return;
    };

    if first_ident.sym.to_string() == "exports" {
      self.has_export = true;
    }

    let MemberProp::Ident(second_ident) = &member.prop else {
      return;
    };
    //
    // module.exports.a = ''
    // module.exports.a = foo
    //
    if first_ident.sym.to_string() == "module" && second_ident.sym.to_string() == "exports" {
      self.has_export = true;
    }
  }
}

impl Visit for WalkerCjs {
  fn visit_member_expr(
    &mut self,
    member: &MemberExpr,
  ) {
    member.visit_children_with(self);
    //
    // module.exports = ''
    //
    self.module_exports(&member)
  }

  fn visit_assign_expr(
    &mut self,
    assign: &AssignExpr,
  ) {
    assign.visit_children_with(self);
    if let AssignTarget::Simple(simple_assign) = &assign.left {
      match &simple_assign {
        //
        // exports = ''
        //
        SimpleAssignTarget::Ident(ident) => {
          if ident.id.sym.to_string() == "exports" {
            self.has_export = true;
          }
        }
        _ => {}
      }
    }
  }

  fn visit_call_expr(
    &mut self,
    call: &CallExpr,
  ) {
    call.visit_children_with(self);
    match &call.callee {
      //
      // require("specifier")
      //
      Callee::Expr(expr) => {
        let Expr::Ident(ident) = &**expr else {
          return;
        };
        if ident.sym != *REQUIRE_SYMBOL {
          return;
        }
        let Expr::Lit(import_specifier_arg) = &*call.args[0].expr else {
          return;
        };
        let Lit::Str(import_specifier) = import_specifier_arg else {
          return;
        };
        self.results.push(ModuleSymbol::ImportCommonjs {
          specifier: import_specifier.value.to_string(),
        });
      }
      _ => {}
    }
  }
}
