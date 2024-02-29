use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use swc_core::atoms::Atom;
use swc_core::common::SourceMap;
use swc_core::common::Span;
use swc_core::ecma::ast::*;

use crate::platform::swc::parse_script;

const JS_DEFINE_EXPORT: &str = include_str!("./js/define_export.js");
const JS_DEFINE_REEXPORT: &str = include_str!("./js/define_reexport.js");
const JS_IMPORT_SCRIPT_CLASSIC: &str = include_str!("./js/import_script_classic.js");
const JS_IMPORT_SCRIPT_ESM: &str = include_str!("./js/import_script_esm.js");
const JS_MANIFEST: &str = include_str!("./js/manifest.js");
const JS_MODULE: &str = include_str!("./js/module.js");
const JS_PRELUDE: &str = include_str!("./js/prelude.js");
const JS_PRELUDE_MACH_REQUIRE: &str = include_str!("./js/prelude_require.js");
const JS_MACH_REQUIRE: &str = include_str!("./js/mach_require.js");
const JS_WRAPPER: &str = include_str!("./js/wrapper.js");

const SYMBOL_EXPORT_DEFAULT_KEY: &str = "default";

pub struct RuntimeFactory {
  decl_define_export: CallExpr,
  decl_define_reexport: CallExpr,
  decl_import_script_classic: Stmt,
  decl_manifest: CallExpr,
  decl_module: Stmt,
  decl_prelude: BlockStmt,
  decl_prelude_mach_require: Vec<Stmt>,
  decl_mach_require: CallExpr,
  decl_wrapper: CallExpr,
}

impl RuntimeFactory {
  pub fn new(source_map: Arc<SourceMap>) -> Self {
    let decl_define_export: CallExpr = {
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

    let decl_define_reexport: CallExpr = {
      let name = PathBuf::from("define_reexport");
      let result = parse_script(&name, JS_DEFINE_REEXPORT, source_map.clone()).unwrap();
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

    let decl_import_script: Stmt = {
      let name = PathBuf::from("import_script");
      let result = parse_script(&name, JS_IMPORT_SCRIPT_CLASSIC, source_map.clone()).unwrap();
      result.script.body[0].to_owned()
    };

    let decl_manifest: CallExpr = {
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

    let decl_module: Stmt = {
      let name = PathBuf::from("module");
      let result = parse_script(&name, JS_MODULE, source_map.clone()).unwrap();
      result.script.body[0].to_owned()
    };

    let decl_prelude: BlockStmt = {
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

    let decl_prelude_mach_require: Vec<Stmt> = {
      let name = PathBuf::from("prelude_mach_require");
      let result = parse_script(&name, JS_PRELUDE_MACH_REQUIRE, source_map.clone()).unwrap();
      vec![result.script.body[0].to_owned(), result.script.body[1].to_owned()]
    };

    let decl_mach_require: CallExpr = {
      let name = PathBuf::from("mach_require");
      let result = parse_script(&name, JS_MACH_REQUIRE, source_map.clone()).unwrap();
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

    let decl_wrapper: CallExpr = {
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
      decl_define_export,
      decl_import_script_classic: decl_import_script,
      decl_manifest,
      decl_module,
      decl_prelude,
      decl_prelude_mach_require,
      decl_mach_require,
      decl_wrapper,
      decl_define_reexport,
    };
  }

  pub fn define_export_default(
    &self,
    expr: Expr,
  ) -> Stmt {
    let mut define_export = self.decl_define_export.clone();

    define_export.args[0].expr = Box::new(Expr::Lit(Lit::Str(Str {
      span: Span::default(),
      value: Atom::from(format!("{}", SYMBOL_EXPORT_DEFAULT_KEY)),
      raw: Some(Atom::from(format!("\"{}\"", SYMBOL_EXPORT_DEFAULT_KEY))),
    })));

    let Expr::Arrow(arrow) = &mut *define_export.args[1].expr else {
      panic!()
    };
    arrow.body = Box::new(BlockStmtOrExpr::Expr(Box::new(expr)));

    Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(Expr::Call(define_export)),
    })
  }

  pub fn define_export_default_named(
    &self,
    export_identifier: &str,
  ) -> Stmt {
    return self.define_export(SYMBOL_EXPORT_DEFAULT_KEY, export_identifier);
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
    self.decl_import_script_classic.clone()
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

  pub fn prelude_mach_require(&self) -> Vec<Stmt> {
    self.decl_prelude_mach_require.clone()
  }

  pub fn mach_require(
    &self,
    bundle_ids: &[&str],
    module_id: &str,
  ) -> Stmt {
    let mut mach_require = self.decl_mach_require.clone();

    mach_require.args[0] = ExprOrSpread {
      spread: None,
      expr: Box::new(Expr::Lit(Lit::Str(Str {
        span: Span::default(),
        value: Atom::from(format!("{}", module_id)),
        raw: Some(Atom::from(format!("\"{}\"", module_id))),
      }))),
    };

    
    if bundle_ids.len() != 0 {
      let Expr::Array(array) = &mut *mach_require.args[1].expr else {
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
    } else {
      mach_require.args.pop();
    }

    Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(Expr::Call(mach_require)),
    })
  }

  pub fn mach_require_awaited(
    &self,
    bundle_ids: &[&str],
    module_id: &str,
  ) -> AwaitExpr {
    let mach_require = self.mach_require(bundle_ids, module_id);

    let Stmt::Expr(mach_require) = mach_require else {
      panic!("Unable to generate import");
    };

    let Expr::Call(mach_require) = *mach_require.expr else {
      panic!("Unable to generate import");
    };

    AwaitExpr {
      span: Span::default(),
      arg: Box::new(Expr::Call(mach_require)),
    }
  }

  /// import { foo } from 'foobar'
  /// import { foo as bar } from 'foobar'
  /// import foo from 'foobar'
  /// import foo, { bar } from 'foobar'
  /// import foo, { foo as bar } from 'foobar'
  pub fn mach_require_named(
    &self,
    bundle_ids: &[&str],
    module_id: &str,
    assignments: Vec<ImportNamed>,
  ) -> Stmt {
    let import_expr = self.mach_require_awaited(bundle_ids, module_id);

    let mut imports = Vec::<ObjectPatProp>::new();

    for assignment in assignments {
      match assignment {
        ImportNamed::Named(name) => imports.push(ObjectPatProp::Assign(AssignPatProp {
          span: Span::default(),
          key: Ident {
            span: Span::default(),
            sym: Atom::from(name.clone()),
            optional: false,
          },
          value: None,
        })),
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
        }
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
        }
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
  pub fn mach_require_namespace(
    &self,
    bundle_ids: &[&str],
    module_id: &str,
    named_as: Option<String>,
  ) -> Stmt {
    let import_expr = self.mach_require_awaited(bundle_ids, module_id);
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

  /// export { foo } from './foo'
  /// export { foo as bar } from './foo'
  pub fn define_reexport_named(
    &self,
    bundle_ids: &[&str],
    module_id: &str,
    keys: &[ImportNamed],
  ) -> Stmt {
    let mut exports = Vec::<Stmt>::new();

    for key in keys {
      match key {
        ImportNamed::Named(key) => {
          exports.push(self.define_export(&key, &format!("lazy.{}", &key)))
        }
        ImportNamed::Renamed(key, key_as) => {
          exports.push(self.define_export(&key_as, &format!("lazy.{}", &key)))
        }
        ImportNamed::Default(_) => todo!(),
      };
    }

    let mach_require = self.mach_require(bundle_ids, module_id);

    let Stmt::Expr(mach_require) = mach_require else {
      panic!("Unable to generate import");
    };

    let Expr::Call(mach_require) = *mach_require.expr else {
      panic!("Unable to generate import");
    };

    let member_expr = MemberExpr {
      span: Span::default(),
      obj: Box::new(Expr::Call(mach_require)),
      prop: MemberProp::Ident(Ident {
        span: Span::default(),
        sym: Atom::from("then"),
        optional: false,
      }),
    };

    let call_expr = CallExpr {
      span: Span::default(),
      callee: Callee::Expr(Box::new(Expr::Member(member_expr))),
      args: vec![ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Arrow(ArrowExpr {
          span: Span::default(),
          params: vec![Pat::Ident(BindingIdent {
            id: Ident {
              span: Span::default(),
              sym: Atom::from("lazy"),
              optional: false,
            },
            type_ann: None,
          })],
          body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
            span: Span::default(),
            stmts: exports,
          })),
          is_async: false,
          is_generator: false,
          type_params: None,
          return_type: None,
        })),
      }],
      type_args: None,
    };

    let await_expr = AwaitExpr {
      span: Span::default(),
      arg: Box::new(Expr::Call(call_expr)),
    };

    return Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(Expr::Await(await_expr)),
    });
  }

  /// export * as foo from './foo'
  /// export * from './foo'
  pub fn define_reexport_namespace(
    &self,
    bundle_ids: &[&str],
    module_id: &str,
    namespace: Option<String>,
  ) -> Stmt {
    let mut define_reexport = self.decl_define_reexport.clone();

    if let Some(namespace) = namespace {
      define_reexport.args.push(ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Lit(Lit::Str(Str {
          span: Span::default(),
          value: Atom::from(format!("{}", namespace)),
          raw: Some(Atom::from(format!("\"{}\"", namespace))),
        }))),
      })
    }

    define_reexport.args[0] = ExprOrSpread {
      spread: None,
      expr: Box::new(Expr::Lit(Lit::Str(Str {
        span: Span::default(),
        value: Atom::from(format!("{}", module_id)),
        raw: Some(Atom::from(format!("\"{}\"", module_id))),
      }))),
    };

    let Expr::Array(array) = &mut *define_reexport.args[1].expr else {
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

    return Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(Expr::Await(AwaitExpr {
        span: Span::default(),
        arg: Box::new(Expr::Call(define_reexport)),
      })),
    });
  }

  /// var foo = ''
  /// let foo = ''
  /// const foo = ''
  pub fn declare_var(
    &self,
    kind: VarDeclKind,
    name: &str,
    expr: Expr,
  ) -> Stmt {
    let decl = VarDecl {
      span: Span::default(),
      kind,
      declare: true,
      decls: vec![VarDeclarator {
        span: Span::default(),
        name: Pat::Ident(BindingIdent {
          id: Ident {
            span: Span::default(),
            sym: Atom::from(name),
            optional: false,
          },
          type_ann: None,
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
