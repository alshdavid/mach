use std::collections::HashMap;

use swc_core::atoms::Atom;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Fold;
use swc_core::ecma::visit::FoldWith;

use crate::kit::swc::lookup_property_access;
use crate::kit::swc::PropAccessType;

pub struct NodeEnvReplacer<'a> {
  pub env: &'a HashMap<String, String>,
}

impl<'a> Fold for NodeEnvReplacer<'a> {
  fn fold_expr(
    &mut self,
    expr: Expr,
  ) -> Expr {
    let expr = expr.fold_children_with(self);

    let Expr::Member(member) = &expr else {
      return expr;
    };
    let Ok(result) = lookup_property_access(&member, &["process", "env"]) else {
      return expr;
    };
    let Some(result) = result else {
      return expr;
    };

    if let PropAccessType::Ident(_, key_name) = &result {
      if let Some(env_var) = self.env.get(key_name) {
        return Expr::Lit(Lit::Str(Str {
          span: DUMMY_SP,
          value: Atom::from(env_var.to_string()),
          raw: None,
        }));
      } else {
        return Expr::Lit(Lit::Null(Null { span: DUMMY_SP }));
      }
    }

    if let PropAccessType::Computed(computed) = &result {
      let Expr::Lit(_) = &computed else {
        return expr;
      };

      // if let Lit::Str(str) = &lit {};

      return expr;
    }

    return expr;
  }
}
