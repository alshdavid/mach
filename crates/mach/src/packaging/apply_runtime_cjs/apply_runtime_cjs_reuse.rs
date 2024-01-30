use once_cell::sync::Lazy;
use swc_core::atoms::Atom;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Fold;

use crate::packaging::runtime::RUNTIME_EXPORT_SYMBOL;

// static MODULE_SYMBOL: Lazy<Atom> = Lazy::new(|| Atom::from("module"));
static EXPORTS_SYMBOL: Lazy<Atom> = Lazy::new(|| Atom::from("exports"));

pub struct ApplyRuntimeCommonJsReuse {}

impl Fold for ApplyRuntimeCommonJsReuse {
  /*
    const foo = exports.a;
    const foo = module.exports.a;
  */
  fn fold_member_expr(
    &mut self,
    n: MemberExpr,
  ) -> MemberExpr {
    let mut n = n.clone();
    let Some(ident) = n.obj.as_mut_ident() else {
      return n;
    };
    if ident.sym != *EXPORTS_SYMBOL {
      return n;
    }
    ident.sym = Atom::from(RUNTIME_EXPORT_SYMBOL);
    return n;
  }
}
