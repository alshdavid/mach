use swc_core::ecma::ast::*;

/// lookup_property_access(&expr, &["module", "exports"])
pub fn lookup_property_access(assign: &mut AssignExpr, keys_test: &[&str]) -> Option<(String, Expr)> {
  let assign = assign.clone();
  let mut keys = Vec::<String>::new();
  let mut pats = Vec::<PatOrExpr>::from(&[assign.left]);

  while let Some(expr) = pats.pop() {
    match expr {
        PatOrExpr::Expr(expr) => {
          match *expr {
            Expr::Member(member) => {
              let MemberProp::Ident(ident) = member.prop else { panic!() };
              pats.push(PatOrExpr::Expr(member.obj));
              keys.push(ident.sym.to_string());
            },
            Expr::Ident(ident) => {
              keys.push(ident.sym.to_string());
            },
            _ => panic!(),
          }
        },
        PatOrExpr::Pat(pat) => {
          let Pat::Expr(expr) = *pat else { panic!() };
          let Expr::Member(member) = *expr else { panic!() };
          let MemberProp::Ident(ident) = member.prop else { panic!() };
          pats.push(PatOrExpr::Expr(member.obj));
          keys.push(ident.sym.to_string());
        },
    }
  };

  keys.reverse();

  if keys.len() - 1 != keys_test.len() {
    return None;
  }

  for (i, key) in keys_test.iter().enumerate() {
    if *key != &keys[i] {
      return None;
    }
  }

  return Some((keys.pop().unwrap(), *assign.right));
}
