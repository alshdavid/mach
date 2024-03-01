use swc_core::ecma::ast::*;

/// lookup_property_access(&expr, &["module", "exports"])
pub fn lookup_property_access(
  assign: &mut AssignExpr,
  keys_test: &[&str],
) -> Option<(Option<String>, Expr, bool)> {
  let assign = assign.clone();
  let mut keys = Vec::<String>::new();
  let mut pats = Vec::<PatOrExpr>::from(&[assign.left]);
  let mut use_quotes = true;

  while let Some(expr) = pats.pop() {
    match expr {
      PatOrExpr::Expr(expr) => match *expr {
        Expr::Member(member) => {
          let MemberProp::Ident(ident) = member.prop else {
            panic!()
          };
          pats.push(PatOrExpr::Expr(member.obj));
          keys.push(ident.sym.to_string());
        }
        Expr::Ident(ident) => {
          keys.push(ident.sym.to_string());
        }
        _ => panic!(),
      },
      PatOrExpr::Pat(pat) => {
        let Pat::Expr(expr) = *pat else { panic!() };
        let Expr::Member(member) = *expr else {
          panic!()
        };
        match member.prop {
          MemberProp::Ident(ident) => {
            pats.push(PatOrExpr::Expr(member.obj));
            keys.push(ident.sym.to_string());
          }
          MemberProp::Computed(computed) => match *computed.expr {
            // module.exports['foo'] = ''
            Expr::Lit(lit) => {
              match lit {
                Lit::Str(str) => {
                  pats.push(PatOrExpr::Expr(member.obj));
                  keys.push(str.value.to_string());
                },
                _ => todo!(),
              }
            },
            // let foo = 'foo'; 
            // module.exports[foo + 'bar'] = ''
            Expr::Ident(ident) => {
              pats.push(PatOrExpr::Expr(member.obj));
              keys.push(ident.sym.to_string());
              use_quotes = false;
            },
            // let foo = 'foo'; 
            // module.exports[foo + 'bar'] = ''
            Expr::Bin(_) => todo!(),
            // let foo = 'foo'; 
            // module.exports[`${foo + 'bar'}`] = ''
            Expr::Tpl(_) => todo!(),
            _ => todo!(),
          },
          MemberProp::PrivateName(_) => todo!(),
        };
      }
    }
  }

  keys.reverse();

  if keys.len() == keys_test.len() {
    for (i, key) in keys_test.iter().enumerate() {
      if *key != &keys[i] {
        return None;
      }
    }
    return Some((None, *assign.right, use_quotes));
  }

  if keys.len() - 1 == keys_test.len() {
    for (i, key) in keys_test.iter().enumerate() {
      if *key != &keys[i] {
        return None;
      }
    }
  
    return Some((Some(keys.pop().unwrap()), *assign.right, use_quotes));
  }

  return None;
}
