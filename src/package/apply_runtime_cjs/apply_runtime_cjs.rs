use std::sync::Arc;

use once_cell::sync::Lazy;
use swc_core::atoms::Atom;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Fold;
use swc_core::ecma::visit::FoldWith;

use crate::bundle::BundleDependencyIndex;
use crate::package::runtime::RuntimeFactory;

static REQUIRE_SYMBOL: Lazy<Atom> = Lazy::new(|| Atom::from("require"));
static MODULE_SYMBOL: Lazy<Atom> = Lazy::new(|| Atom::from("module"));
static EXPORTS_SYMBOL: Lazy<Atom> = Lazy::new(|| Atom::from("exports"));

pub struct ApplyRuntimeCommonJs {
  pub asset_id: String,
  pub dependency_index: Arc<BundleDependencyIndex>,
  pub runtime_factory: Arc<RuntimeFactory>,
}

impl Fold for ApplyRuntimeCommonJs {
  fn fold_call_expr(&mut self, call_expr: CallExpr) -> CallExpr {
    let call_expr = call_expr.fold_children_with(self);
    match &call_expr.callee {
      /*
         require("specifier")
      */
      Callee::Expr(expr) => {
        let Expr::Ident(ident) = &**expr else {
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

        let (asset_id, _) = self
          .dependency_index
          .get(&(
            self.asset_id.to_string(),
            import_specifier.value.to_string(),
          ))
          .unwrap();
        return self.runtime_factory.import_call_expr(&asset_id);
      }
      Callee::Import(_) => {}
      Callee::Super(_) => {}
    };

    return call_expr;
  }

  /*
    module.exports.a = ''
    exports.a = ''
  */
  fn fold_assign_expr(&mut self, assign_expr: AssignExpr) -> AssignExpr {
    let assign_expr = assign_expr.fold_children_with(self);

    let v = assign_expr.clone();
    let PatOrExpr::Pat(expr) = assign_expr.left else {
      return v;
    };

    let Pat::Expr(expr) = *expr else {
      return v;
    };

    let Expr::Member(top) = *expr else {
      return v;
    };

    match *top.obj {
      // module.exports.a = ''
      Expr::Member(expr) => {
        let Expr::Ident(first) = *expr.obj else {
          return v;
        };

        if first.sym != *MODULE_SYMBOL {
          return v;
        }

        let MemberProp::Ident(second) = expr.prop else {
          return v;
        };

        if second.sym != *EXPORTS_SYMBOL {
          return v;
        };

        match top.prop {
          MemberProp::Ident(ident) => {
            return self
              .runtime_factory
              .require_export_named(ident.sym.as_str(), v.right)
              .as_expr()
              .unwrap()
              .expr
              .as_assign()
              .unwrap()
              .clone();
          }
          MemberProp::Computed(_) => todo!(),
          MemberProp::PrivateName(_) => {
            return v;
          }
        }
      }
      // exports.a = ''
      // module.exports = require('./a')
      Expr::Ident(ident) => {
        if ident.sym == *MODULE_SYMBOL {
          if let MemberProp::Ident(right) = top.prop {
            if right.sym == *EXPORTS_SYMBOL {
              return self
                .runtime_factory
                .require_export_default(v.right)
                .as_expr()
                .unwrap()
                .expr
                .as_assign()
                .unwrap()
                .clone();
            }
          }
          return v;
        }

        if ident.sym != *EXPORTS_SYMBOL {
          return v;
        };

        match top.prop {
          MemberProp::Ident(ident) => {
            return self
              .runtime_factory
              .require_export_named(ident.sym.as_str(), v.right)
              .as_expr()
              .unwrap()
              .expr
              .as_assign()
              .unwrap()
              .clone();
          }
          MemberProp::Computed(_) => todo!(),
          MemberProp::PrivateName(_) => {
            return v;
          }
        }
      }
      _ => {
        return v;
      }
    }
  }
}
