/*

mod adapters;
mod args;
mod bundling;
mod config;
mod packaging;
mod platform;
mod plugins;
mod public;
mod transformation;
mod emit;

use std::sync::Arc;

use swc_core::{atoms::Atom, common::{SourceMap, Span}, ecma::ast::{BlockStmt, BlockStmtOrExpr, VarDeclKind}};
use swc_core::ecma::ast::*;

use crate::{packaging::runtime_factory::{self, ImportNamed}, platform::swc::render_stmts};

fn main() {
  let rf = runtime_factory::RuntimeFactory::new(Arc::new(SourceMap::default()));

  let stmt = rf.define_reexport_namespace(
    Some("namespace".to_string()),
    // None,
    "module_id",
    &[],
  );

  // let stmt = rf.define_reexport_named(
  //   &[
  //     ImportNamed::Named("a".to_string()),
  //     ImportNamed::Renamed("a".to_string(), "b".to_string()),
  //   ],
  //   "module_id",
  //   &[],
  // );

  // let stmt = rf.mach_require(
  //   "module_id",
  //   &[],
  //   Some(BlockStmtOrExpr::BlockStmt(BlockStmt{
  //       span: Span::default(),
  //       stmts: vec![
  //         rf.declare_var(VarDeclKind::Const, "foo", Expr::Lit(Lit::Str(Str {
  //           span: Span::default(),
  //           value: Atom::from("hi"),
  //           raw: Some(Atom::from(format!("\"hi\""))),
  //         })))
  //       ],
  //   })),
  // );

  let rendered = render_stmts(&vec![stmt], Arc::new(SourceMap::default()));

  println!("{}", rendered);
}


*/