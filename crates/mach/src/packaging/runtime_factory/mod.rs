use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use swc_core::atoms::Atom;
use swc_core::common::SourceMap;
use swc_core::common::Span;
use swc_core::ecma::ast::*;

use crate::platform::swc::parse_script;

const JS_DEFINE_EXPORT: &str = include_str!("./js/define_export.js");
const JS_IMPORT_SCRIPT: &str = include_str!("./js/import_script.js");
const JS_MANIFEST: &str = include_str!("./js/manifest.js");
const JS_MODULE: &str = include_str!("./js/module.js");
const JS_PRELUDE: &str = include_str!("./js/prelude.js");
const JS_PRELUDE_REQUIRE_ASYNC: &str = include_str!("./js/prelude_require_async.js");
const JS_REQUIRE_ASYNC: &str = include_str!("./js/require_async.js");
const JS_WRAPPER: &str = include_str!("./js/wrapper.js");

const SYMBOL_EXPORT_DEFAULT_KEY: &str = "default";

pub struct RuntimeFactory {
  decl_define_export: CallExpr,
  decl_import_script: Stmt,
  decl_manifest: CallExpr,
  decl_module: Stmt,
  decl_prelude: BlockStmt,
  decl_prelude_require_async: Stmt,
  decl_require_async: CallExpr,
  decl_wrapper: CallExpr,
}

impl RuntimeFactory {
  pub fn new(source_map: Arc<SourceMap>) -> Self {
    let define_export: CallExpr = {
      let name = PathBuf::from("define_export");
      let result = parse_script(&name, JS_DEFINE_EXPORT, source_map.clone()).unwrap();
      result.script.body[0]
        .to_owned()
        .as_expr()
        .unwrap()
        .to_owned()
        .expr
        .as_call()
        .unwrap()
        .to_owned()
    };

    let import_script: Stmt = {
      let name = PathBuf::from("import_script");
      let result = parse_script(&name, JS_IMPORT_SCRIPT, source_map.clone()).unwrap();
      result.script.body[0].to_owned()
    };

    let manifest: CallExpr = {
      let name = PathBuf::from("manifest");
      let result = parse_script(&name, JS_MANIFEST, source_map.clone()).unwrap();
      result.script.body[0]
        .to_owned()
        .as_expr()
        .unwrap()
        .to_owned()
        .expr
        .as_call()
        .unwrap()
        .to_owned()
    };

    let module: Stmt = {
      let name = PathBuf::from("module");
      let result = parse_script(&name, JS_MODULE, source_map.clone()).unwrap();
      result.script.body[0].to_owned()
    };

    let prelude: BlockStmt = {
      let name = PathBuf::from("prelude");
      let result = parse_script(&name, JS_PRELUDE, source_map.clone()).unwrap();
      let mut block_stmt = BlockStmt {
        span: Span::default(),
        stmts: vec![],
      };
      for stmt in result.script.body {
        block_stmt.stmts.push(stmt);
      }
      block_stmt
    };

    let prelude_require_async: Stmt = {
      let name = PathBuf::from("prelude_require_async");
      let result = parse_script(&name, JS_PRELUDE_REQUIRE_ASYNC, source_map.clone()).unwrap();
      result.script.body[0].to_owned()
    };

    let require_async: CallExpr = {
      let name = PathBuf::from("require_async");
      let result = parse_script(&name, JS_REQUIRE_ASYNC, source_map.clone()).unwrap();
      result.script.body[0]
        .to_owned()
        .as_expr()
        .unwrap()
        .to_owned()
        .expr
        .as_call()
        .unwrap()
        .to_owned()
    };

    let wrapper: CallExpr = {
      let name = PathBuf::from("wrapper");
      let result = parse_script(&name, JS_WRAPPER, source_map.clone()).unwrap();
      result.script.body[0]
        .to_owned()
        .as_expr()
        .unwrap()
        .to_owned()
        .expr
        .as_call()
        .unwrap()
        .to_owned()
    };

    return Self {
      decl_define_export: define_export,
      decl_import_script: import_script,
      decl_manifest: manifest,
      decl_module: module,
      decl_prelude: prelude,
      decl_prelude_require_async: prelude_require_async,
      decl_require_async: require_async,
      decl_wrapper: wrapper,
    };
  }

  pub fn define_export(
    &self,
    export_key: &str,
    export_identifier: &str,
  ) -> Stmt {
    let mut define_export = self.decl_define_export.clone();

    define_export.args[0].expr = Box::new(Expr::Lit(Lit::Str(Str {
      span: Span::default(),
      value: Atom::from(format!("{}", export_key)),
      raw: Some(Atom::from(format!("\"{}\"", export_key))),
    })));

    let Expr::Arrow(arrow) = &mut *define_export.args[1].expr else {
      panic!()
    };
    let BlockStmtOrExpr::Expr(block) = &mut *arrow.body else {
      panic!()
    };
    let Expr::Ident(ident) = &mut **block else {
      panic!()
    };
    ident.sym = Atom::from(export_identifier);

    Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(Expr::Call(define_export)),
    })
  }

  pub fn import_script(&self) -> Stmt {
    self.decl_import_script.clone()
  }

  pub fn manifest(
    &self,
    bundles: &HashMap<String, String>,
  ) -> Result<Stmt, String> {
    let mut manifest = self.decl_manifest.clone();

    let Ok(data) = serde_json::to_string(bundles) else {
      return Err("Unable to parse JSON".to_string());
    };

    let callee = &mut manifest.args[1].expr.as_mut_call().unwrap();

    callee.args[0].expr = Box::new(Expr::Lit(Lit::Str(Str {
      span: Span::default(),
      value: Atom::from(format!("{}", data)),
      raw: Some(Atom::from(format!("{}", data))),
    })));

    Ok(Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(Expr::Call(manifest)),
    }))
  }

  pub fn module(
    &self,
    module_id: &str,
    contents: Vec<Stmt>,
  ) -> Stmt {
    let mut module = self.decl_module.clone();

    let Stmt::Expr(expr) = &mut module else {
      panic!()
    };
    let Expr::Assign(assign) = &mut *expr.expr else {
      panic!()
    };
    {
      let PatOrExpr::Pat(pat) = &mut assign.left else {
        panic!()
      };
      let Pat::Expr(expr) = &mut **pat else {
        panic!()
      };
      let Expr::Member(member) = &mut **expr else {
        panic!()
      };
      let MemberProp::Computed(prop) = &mut member.prop else {
        panic!()
      };

      prop.expr = Box::new(Expr::Lit(Lit::Str(Str {
        span: Span::default(),
        value: Atom::from(format!("{}", module_id)),
        raw: Some(Atom::from(format!("\"{}\"", module_id))),
      })));
    }

    {
      let Expr::Arrow(arrow) = &mut *assign.right else {
        panic!()
      };
      arrow.body = Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
        span: Span::default(),
        stmts: contents.to_vec(),
      }));
    }

    module
  }

  pub fn prelude(
    &self,
    project_identifier: &str,
  ) -> Vec<Stmt> {
    let mut prelude = self.decl_prelude.clone();

    let Stmt::Decl(decl) = &mut prelude.stmts[0] else {
      panic!();
    };
    let Decl::Var(var) = &mut *decl else {
      panic!();
    };
    let Some(decl) = &mut var.decls[0].init else {
      panic!();
    };
    let Expr::Assign(assign) = &mut **decl else {
      panic!();
    };

    {
      let PatOrExpr::Pat(pat) = &mut assign.left else {
        panic!();
      };
      let Pat::Expr(expr) = &mut **pat else {
        panic!();
      };
      let Expr::Member(member) = &mut **expr else {
        panic!();
      };
      let MemberProp::Computed(prop) = &mut member.prop else {
        panic!();
      };
      prop.expr = Box::new(Expr::Lit(Lit::Str(Str {
        span: Span::default(),
        value: Atom::from(format!("{}", project_identifier)),
        raw: Some(Atom::from(format!("{}", project_identifier))),
      })));
    }

    {
      let Expr::Bin(bin) = &mut *assign.right else {
        panic!();
      };
      let Expr::Member(member) = &mut *bin.left else {
        panic!();
      };
      let MemberProp::Computed(prop) = &mut member.prop else {
        panic!();
      };
      prop.expr = Box::new(Expr::Lit(Lit::Str(Str {
        span: Span::default(),
        value: Atom::from(format!("{}", project_identifier)),
        raw: Some(Atom::from(format!("{}", project_identifier))),
      })));
    }

    prelude.stmts
  }

  pub fn prelude_require_async(&self) -> Stmt {
    self.decl_prelude_require_async.clone()
  }

  pub fn require_async(
    &self,
    bundle_ids: &[&str],
    module_id: &str,
  ) -> Stmt {
    let mut require_async = self.decl_require_async.clone();

    let Expr::Array(array) = &mut *require_async.args[0].expr else {
      panic!()
    };
    for bundle_id in bundle_ids {
      array.elems.push(Some(ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Lit(Lit::Str(Str {
          span: Span::default(),
          value: Atom::from(format!("{}", bundle_id)),
          raw: Some(Atom::from(format!("\"{}\"", bundle_id))),
        }))),
      }))
    }

    require_async.args[1] = ExprOrSpread {
      spread: None,
      expr: Box::new(Expr::Lit(Lit::Str(Str {
        span: Span::default(),
        value: Atom::from(format!("{}", module_id)),
        raw: Some(Atom::from(format!("\"{}\"", module_id))),
      }))),
    };

    Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(Expr::Call(require_async)),
    })
  }

  pub fn require_async_awaited(
    &self,
    bundle_ids: &[&str],
    module_id: &str,
  ) -> AwaitExpr {
    let require_async = self.require_async(bundle_ids, module_id);

    let Stmt::Expr(require_async) = require_async else {
      panic!("Unable to generate import");
    };

    let Expr::Call(require_async) = *require_async.expr else {
      panic!("Unable to generate import");
    };

    AwaitExpr {
        span: Span::default(),
        arg: Box::new(Expr::Call(require_async)),
    }
  }
  /// import { foo } from 'foobar'
  /// import { foo as bar } from 'foobar'
  /// import foo from 'foobar'
  /// import foo, { bar } from 'foobar'
  /// import foo, { foo as bar } from 'foobar'
  pub fn require_async_named(
    &self, 
    bundle_ids: &[&str],
    module_id: &str,
    assignments: Vec<ImportNamed>) -> Stmt {
    let import_expr = self.require_async_awaited(bundle_ids, module_id);

    let mut imports = Vec::<ObjectPatProp>::new();

    for assignment in assignments {
      match assignment {
        ImportNamed::Named(name) => {
          imports.push(ObjectPatProp::Assign(AssignPatProp {
            span: Span::default(),
            key: Ident {
              span: Span::default(),
              sym: Atom::from(name.clone()),
              optional: false,
            },
            value: None,
          }))
        },
        ImportNamed::Renamed(key, key_as) => {
          imports.push(ObjectPatProp::KeyValue(KeyValuePatProp {
            key: PropName::Ident(Ident {
              span: Span::default(),
              sym: Atom::from(key.clone()),
              optional: false,
            }),
            value: Box::new(Pat::Ident(BindingIdent {
              id: Ident {
                span: Span::default(),
                sym: Atom::from(key_as.clone()),
                optional: false,
              },
              type_ann: None,
            })),
          }));
        },
        ImportNamed::Default(name) => {
          imports.push(ObjectPatProp::KeyValue(KeyValuePatProp {
            key: PropName::Computed(ComputedPropName {
              span: Span::default(),
              expr: Box::new(Expr::Lit(Lit::Str(Str {
                span: Span::default(),
                value: Atom::from(SYMBOL_EXPORT_DEFAULT_KEY),
                raw: Some(Atom::from(format!("\"{}\"", SYMBOL_EXPORT_DEFAULT_KEY))),
              }))),
            }),
            value: Box::new(Pat::Ident(BindingIdent {
              id: Ident {
                span: Span::default(),
                sym: Atom::from(name.clone()),
                optional: false,
              },
              type_ann: None,
            })),
          }));
        },
      }
    }
    
    return Stmt::Decl(Decl::Var(Box::new(VarDecl {
      span: Span::default(),
      kind: VarDeclKind::Const,
      declare: false,
      decls: vec![VarDeclarator {
        span: Span::default(),
        name: Pat::Object(ObjectPat {
          span: Span::default(),
          props: imports,
          optional: false,
          type_ann: None,
        }),
        init: Some(Box::new(Expr::Await(import_expr))),
        definite: false,
      }],
    })));
  }

  /// import 'foobar'
  /// import * as foobar from 'foobar'
  pub fn require_async_namespace(
    &self,
    bundle_ids: &[&str],
    module_id: &str,
    named_as: Option<String>,
  ) -> Stmt {
    let import_expr = self.require_async_awaited(bundle_ids, module_id);
    let await_expr = ExprStmt {
      span: Span::default(),
      expr: Box::new(Expr::Await(import_expr)),
    };

    let Some(assignment) = named_as else {
      return Stmt::Expr(await_expr);
    };

    return Stmt::Decl(Decl::Var(Box::new(VarDecl {
      span: Span::default(),
      kind: VarDeclKind::Const,
      declare: false,
      decls: vec![VarDeclarator {
        span: Span::default(),
        name: Pat::Ident(BindingIdent {
          id: Ident {
            span: Span::default(),
            sym: Atom::from(assignment),
            optional: false,
          },
          type_ann: None,
        }),
        init: Some(await_expr.expr),
        definite: false,
      }],
    })));
  }

  pub fn wrapper(
    &self,
    stmts: Vec<Stmt>,
  ) -> Stmt {
    let mut wrapper = self.decl_wrapper.clone();

    let Callee::Expr(expr) = &mut wrapper.callee else {
      panic!()
    };
    let Expr::Paren(paren) = &mut **expr else {
      panic!()
    };
    let Expr::Arrow(arrow) = &mut *paren.expr else {
      panic!()
    };
    let BlockStmtOrExpr::BlockStmt(block) = &mut *arrow.body else {
      panic!()
    };
    let current_body = std::mem::take(&mut block.stmts);

    for stmt in stmts {
      block.stmts.push(stmt.clone());
    }

    for stmt in current_body {
      block.stmts.push(stmt.clone());
    }

    Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(Expr::Call(wrapper)),
    })
  }

  pub fn declare_var(&self, kind: VarDeclKind, name: &str, expr: Expr) -> Stmt {
    let decl = VarDecl {
        span: Span::default(),
        kind,
        declare: true,
        decls: vec![VarDeclarator{ 
          span: Span::default(), 
          name: Pat::Ident(BindingIdent { 
            id: Ident { 
              span: Span::default(),
              sym: Atom::from(name),
              optional: false, 
            }, 
            type_ann: None 
          }), 
          init: Some(Box::new(expr)), 
          definite: true, 
        }],
    };
    return Stmt::Decl(Decl::Var(Box::new(decl)));
  }
}

#[derive(Debug, Clone)]
pub enum ImportNamed {
  /// import { foo } from 'specifier'
  ///          ---
  Named(String),
  /// import { foo as bar } from 'specifier'
  ///          ----------
  Renamed(String, String),
  /// import foo from 'specifier'
  ///        ---
  Default(String),
}

pub type ExportNamed = ImportNamed;
