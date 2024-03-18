use swc_core::ecma::ast::*;

#[derive(Debug)]
pub enum PropAccessType {
  Ident(Ident, String),
  Computed(Expr),
}

/// lookup_property_access(&expr, &["module", "exports"])
/// module.exports = 'foo'      // Ok(None)
/// module.exports              // Ok(None)
/// module.exports.foo = 'foo'  // Ok(PropertyAccessType::Ident(ident, "foo"))
/// module.exports.foo          // Ok(PropertyAccessType::Ident(ident, "foo"))
/// module.notexports.foo       // Err(())
pub fn lookup_property_access(
  input: &MemberExpr,
  source_keys: &[&str],
) -> Result<Option<PropAccessType>, ()> {
  // TODO optimize
  let input = input.clone();

  let mut keys = Vec::<PropAccessType>::new();
  let mut pats = Vec::<MemberExpr>::from(&[input]);

  // Flatten tree
  while let Some(member) = pats.pop() {
    match member.prop {
      MemberProp::Ident(ident) => {
        let value = ident.sym.to_string();
        keys.push(PropAccessType::Ident(ident, value));
      }
      MemberProp::Computed(computed) => {
        keys.push(PropAccessType::Computed(*computed.expr));
      }
      MemberProp::PrivateName(_) => todo!(),
    }

    match *member.obj {
      Expr::Member(member) => {
        pats.push(member);
      }
      Expr::Ident(ident) => {
        let value = ident.sym.to_string();
        keys.push(PropAccessType::Ident(ident, value));
      }
      _ => {}
    }
  }

  let source_keys_count = source_keys.len();
  let exprs_count = keys.len();

  if exprs_count != source_keys_count && exprs_count != source_keys_count + 1 {
    return Err(());
  }

  // loop in reverse order
  let mut count = 0;
  while let Some(access_type) = keys.pop() {
    let source_key = source_keys.get(count);

    match &access_type {
      PropAccessType::Ident(ident, _) => {
        if count == source_keys.len() {
          return Ok(Some(access_type));
        }
        if *source_key.unwrap() != ident.sym.to_string() {
          return Err(());
        }
      }
      PropAccessType::Computed(expr) => match expr {
        Expr::Ident(ident) => {
          if count == source_keys.len() {
            return Ok(Some(access_type));
          }
          if *source_key.unwrap() != ident.sym.to_string() {
            return Err(());
          }
        }
        Expr::Lit(lit) => match lit {
          Lit::Str(str) => {
            if count == source_keys.len() {
              return Ok(Some(access_type));
            }
            if *source_key.unwrap() != str.value.to_string() {
              return Err(());
            }
          }
          _ => {
            if count == source_keys.len() {
              return Ok(Some(access_type));
            }
          }
        },
        _ => {
          if count == source_keys.len() {
            return Ok(Some(access_type));
          }
        }
      },
    }

    count += 1
  }

  return Ok(None);
}

/*

use std::{path::PathBuf, sync::Arc};

use swc_core::common::SourceMap;
use swc_core::ecma::ast::*;

use crate::{kit::swc::{lookup_property_access, parse_script}, public::Config};

use super::BuildCommand;

pub fn main(command: BuildCommand) {

  let code = "module.exports";
  let mut stmt = parse_script(&PathBuf::from(""), code, Arc::new(SourceMap::default())).unwrap().script;
  let Stmt::Expr(expr) = stmt.body.pop().unwrap() else { panic!(); };

  let member: MemberExpr = 'block: {
    if let Expr::Member(member) = *expr.expr {
      break 'block member;
    }
    if let Expr::Assign(assign) = *expr.expr {
      let AssignTarget::Simple(pat) = assign.left else { panic!(); };
      let SimpleAssignTarget::Member(member) = pat else { panic!() };
      break 'block member
    }
    panic!();
  };

  let res = lookup_property_access(&member, &["module", "exports"]).unwrap();

  dbg!(&res);
}

*/
