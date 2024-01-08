use std::path::PathBuf;

use swc_core::atoms::Atom;
use swc_core::common::sync::Lrc;
use swc_core::common::SourceMap;
use swc_core::common::Span;
use swc_core::ecma::ast::*;

use crate::default_plugins::transformers::javascript::parse_module;

/*
  This utility contains the wrapping JavaScript code that
  parsed modules are transformed into
*/

pub const MODULE_HEADER: &str = include_str!("./sources/module_header.js");
pub const MODULE_WRAPPER: &str = include_str!("./sources/module_wrapper.js");
pub const IMPORT: &str = include_str!("./sources/import.js");
pub const IMPORT_DYNAMIC: &str = include_str!("./sources/import_dynamic.js");
pub const EXPORT_ALL: &str = include_str!("./sources/export_all.js");
pub const EXPORT_CJS: &str = include_str!("./sources/export_cjs.js");
pub const BOOTSTRAP: &str = include_str!("./sources/bootstrap.js");

pub const RUNTIME_DEFAULT_EXPORT_SYMBOL: &str = "default";
pub const RUNTIME_EXPORT_SYMBOL: &str = "$$export";

/// RuntimeFactory mints SWC AST statements that represents module
/// syntax that is transformed into the equivalent bundle wrapping code
///
/// EXAMPLE: import * as foo from './bar'
/// BECOMES: const foo = __mach_import_module('./bar')
pub struct RuntimeFactory {
  module_header_stmt: Stmt,
  module_wrapper_stmt: Stmt,
  bootstrap_stmt: ExprStmt,
  import_stmt: CallExpr,
  import_dynamic_stmt: CallExpr,
  export_all_stmt: Stmt,
  export_all_require_stmt: Stmt,
}

impl RuntimeFactory {
  pub fn new(source_map: Lrc<SourceMap>) -> Self {
    let module_header_stmt: Stmt = {
      let name = PathBuf::from("mach_header");
      let (module, _) = parse_module(&name, MODULE_HEADER, source_map.clone()).unwrap();
      module.body[0].as_stmt().unwrap().to_owned()
    };

    let module_wrapper_stmt: Stmt = {
      let name = PathBuf::from("mach_wrapper");
      let (module, _) = parse_module(&name, MODULE_WRAPPER, source_map.clone()).unwrap();
      module.body[0].as_stmt().unwrap().to_owned()
    };

    let bootstrap_stmt: ExprStmt = {
      let name = PathBuf::from("mach_bootstrap");
      let (module, _) = parse_module(&name, BOOTSTRAP, source_map.clone()).unwrap();
      let stmt = module.body[0].as_stmt().unwrap().to_owned();
      stmt.as_expr().unwrap().to_owned()
    };

    let import_stmt: CallExpr = {
      let name = PathBuf::from("mach_import");
      let (module, _) = parse_module(&name, IMPORT, source_map.clone()).unwrap();
      let stmt = module.body[0].as_stmt().unwrap().to_owned();
      let expr = stmt.as_expr().unwrap().to_owned();
      expr.expr.as_call().unwrap().to_owned()
    };

    let import_dynamic_stmt: CallExpr = {
      let name = PathBuf::from("mach_import_dynamic");
      let (module, _) = parse_module(&name, IMPORT_DYNAMIC, source_map.clone()).unwrap();
      let stmt = module.body[0].as_stmt().unwrap().to_owned();
      let expr = stmt.as_expr().unwrap().to_owned();
      expr.expr.as_call().unwrap().to_owned()
    };

    let export_all_stmt: Stmt = {
      let name = PathBuf::from("mach_export_all");
      let (module, _) = parse_module(&name, EXPORT_ALL, source_map.clone()).unwrap();
      module.body[0].as_stmt().unwrap().to_owned()
    };

    let export_all_require_stmt: Stmt = {
      let name = PathBuf::from("mach_export_cjs");
      let (module, _) = parse_module(&name, EXPORT_CJS, source_map.clone()).unwrap();
      module.body[0].as_stmt().unwrap().to_owned()
    };

    return RuntimeFactory {
      module_header_stmt,
      module_wrapper_stmt,
      bootstrap_stmt,
      import_stmt,
      import_dynamic_stmt,
      export_all_stmt,
      export_all_require_stmt,
    };
  }

  /// The header
  pub fn header(&self) -> Stmt {
    return self.module_header_stmt.clone();
  }

  pub fn bootstrap(&self, entry_module_id: &str) -> Stmt {
    let mut template = self.bootstrap_stmt.clone();

    let arg = template.expr.as_mut_unary().unwrap();
    let arg = arg.arg.as_mut_call().unwrap();
    let callee = arg.callee.as_mut_expr().unwrap();
    let func = callee.as_mut_fn_expr().unwrap();
    let block = func.function.body.as_mut().unwrap();
    let expr = block.stmts[3].as_mut_expr().unwrap();
    let call = expr.expr.as_mut_call().unwrap();

    call.args[0].expr = Box::new(Expr::Lit(Lit::Str(Str {
      span: Span::default(),
      value: Atom::from(entry_module_id),
      raw: Some(Atom::from(format!("\"{}\"", entry_module_id))),
    })));

    return Stmt::Expr(template);
  }

  pub fn bootstrap_script(&self, entry_module_id: &str) -> Script {
    return Script{ 
      span: Span::default(),
      body: vec![self.bootstrap(entry_module_id)],
      shebang: None, 
    };
  }

  /// Mints a module wrapper
  pub fn module(&self, specifier: &str, has_exports: bool, body: Vec<Stmt>) -> Stmt {
    let mut stmt = self.module_wrapper_stmt.clone();

    let Stmt::Expr(expr) = &mut stmt else {
      panic!("Unable to generate module");
    };

    let Expr::Assign(expr) = expr.expr.as_mut() else {
      panic!("Unable to generate module");
    };

    {
      let PatOrExpr::Pat(expr) = &mut expr.left else {
        panic!("Unable to generate module");
      };

      let Pat::Expr(expr) = expr.as_mut() else {
        panic!("Unable to generate module");
      };

      let Expr::Member(expr) = expr.as_mut() else {
        panic!("Unable to generate module");
      };

      let MemberProp::Computed(expr) = &mut expr.prop else {
        panic!("Unable to generate module");
      };

      expr.expr = Box::new(Expr::Lit(Lit::Str(Str {
        span: Span::default(),
        value: Atom::from(specifier),
        raw: Some(Atom::from(format!("\"{}\"", specifier))),
      })));
    }

    {
      let Expr::Arrow(expr) = expr.right.as_mut() else {
        panic!("Unable to generate module");
      };

      let mut stmt_body = Vec::<Stmt>::new();

      for item in body {
        stmt_body.push(item);
      }

      expr.body = Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
        span: Span::default(),
        stmts: stmt_body,
      }));

      if !has_exports {
        expr.params = vec![];
      }
    }

    return stmt;
  }

  /// Mints an import statement with no assignment
  /// import "./foo"
  pub fn import(&self, specifier: &str) -> Stmt {
    let expr = self.import_call_expr(specifier);

    return Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(Expr::Call(expr)),
    });
  }

  pub fn import_call_expr(&self, specifier: &str) -> CallExpr {
    let mut expr = self.import_stmt.clone();

    let arg = &mut expr.args[0];

    arg.expr = Box::new(Expr::Lit(Lit::Str(Str {
      span: Span::default(),
      value: Atom::from(specifier),
      raw: Some(Atom::from(format!("\"{}\"", specifier))),
    })));

    return expr;
  }

  pub fn import_dynamic_call_expr(&self, specifier: &str) -> CallExpr {
    let mut expr = self.import_dynamic_stmt.clone();

    let promise_arg = &mut expr.args[0];

    let Expr::Call(import_call) = &mut *promise_arg.expr else {
      panic!("Invalid")
    };

    import_call.args[0].expr = Box::new(Expr::Lit(Lit::Str(Str {
      span: Span::default(),
      value: Atom::from(specifier),
      raw: Some(Atom::from(format!("\"{}\"", specifier))),
    })));

    return expr;
  }

  /// Mints an import statement with named imports assigned
  /// import { bar } from './foo'
  pub fn import_named(&self, specifier: &str, assignments: &Vec<ImportNamed>) -> Stmt {
    let mut imports = Vec::<ObjectPatProp>::new();

    for assignment in assignments {
      match assignment {
        ImportNamed::Key(name) => imports.push(ObjectPatProp::Assign(AssignPatProp {
          span: Span::default(),
          key: Ident {
            span: Span::default(),
            sym: Atom::from(name.clone()),
            optional: false,
          },
          value: None,
        })),
        ImportNamed::Renamed(name, rename) => {
          imports.push(ObjectPatProp::KeyValue(KeyValuePatProp {
            key: PropName::Ident(Ident {
              span: Span::default(),
              sym: Atom::from(name.clone()),
              optional: false,
            }),
            value: Box::new(Pat::Ident(BindingIdent {
              id: Ident {
                span: Span::default(),
                sym: Atom::from(rename.clone()),
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
                value: Atom::from(RUNTIME_DEFAULT_EXPORT_SYMBOL),
                raw: Some(Atom::from(format!("\"{}\"", RUNTIME_DEFAULT_EXPORT_SYMBOL))),
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

    let import_expr = self.import(specifier);

    let Stmt::Expr(import_expr) = import_expr else {
      panic!("Unable to generate import");
    };

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
        init: Some(import_expr.expr),
        definite: false,
      }],
    })));
  }

  /// Mints an import statement with all exports assigned to a single variable
  /// import * as foo from './foo'
  pub fn import_star(&self, specifier: &str, assignment: &str) -> Stmt {
    let import_expr = self.import(specifier);

    let Stmt::Expr(import_expr) = import_expr else {
      panic!("Unable to generate import");
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
        init: Some(import_expr.expr),
        definite: false,
      }],
    })));
  }

  /// Mints an export statement of a previously declared identifier
  /// export { foo }
  pub fn export(&self, key: &str) -> Stmt {
    return self.export_renamed(key, key);
  }

  /// Mints an export statement of a previously declared identifier, renaming the export
  /// export { foo as bar }
  pub fn export_renamed(&self, key: &str, exported_as: &str) -> Stmt {
    let stmt = Expr::Assign(AssignExpr {
      span: Span::default(),
      op: AssignOp::Assign,
      left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
        span: Span::default(),
        obj: Box::new(Expr::Ident(Ident {
          span: Span::default(),
          sym: Atom::from(RUNTIME_EXPORT_SYMBOL),
          optional: false,
        })),
        prop: MemberProp::Computed(ComputedPropName {
          span: Span::default(),
          expr: Box::new(Expr::Lit(Lit::Str(Str {
            span: Span::default(),
            value: Atom::from(exported_as),
            raw: Some(Atom::from(format!("\"{}\"", exported_as))),
          }))),
        }),
      }))),
      right: Box::new(Expr::Ident(Ident {
        span: Span::default(),
        sym: Atom::from(key),
        optional: false,
      })),
    });
    return Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(stmt),
    });
  }

  /// Mints a default export for a previously declared identifier
  /// export default foo
  pub fn export_default(&self, key: &str) -> Stmt {
    let stmt = Expr::Assign(AssignExpr {
      span: Span::default(),
      op: AssignOp::Assign,
      left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
        span: Span::default(),
        obj: Box::new(Expr::Ident(Ident {
          span: Span::default(),
          sym: Atom::from(RUNTIME_EXPORT_SYMBOL),
          optional: false,
        })),
        prop: MemberProp::Computed(ComputedPropName {
          span: Span::default(),
          expr: Box::new(Expr::Lit(Lit::Str(Str {
            span: Span::default(),
            value: Atom::from(RUNTIME_DEFAULT_EXPORT_SYMBOL),
            raw: Some(Atom::from(format!("\"{}\"", RUNTIME_DEFAULT_EXPORT_SYMBOL))),
          }))),
        }),
      }))),
      right: Box::new(Expr::Ident(Ident {
        span: Span::default(),
        sym: Atom::from(key),
        optional: false,
      })),
    });
    return Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(stmt),
    });
  }

  /// Mints a default export for an anonymous expression, like a class or function
  /// export default 'hello'
  pub fn export_default_expr(&self, stmt: Box<Expr>) -> Stmt {
    let stmt = Expr::Assign(AssignExpr {
      span: Span::default(),
      op: AssignOp::Assign,
      left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
        span: Span::default(),
        obj: Box::new(Expr::Ident(Ident {
          span: Span::default(),
          sym: Atom::from(RUNTIME_EXPORT_SYMBOL),
          optional: false,
        })),
        prop: MemberProp::Computed(ComputedPropName {
          span: Span::default(),
          expr: Box::new(Expr::Lit(Lit::Str(Str {
            span: Span::default(),
            value: Atom::from(RUNTIME_DEFAULT_EXPORT_SYMBOL),
            raw: Some(Atom::from(format!("\"{}\"", RUNTIME_DEFAULT_EXPORT_SYMBOL))),
          }))),
        }),
      }))),
      right: stmt,
    });
    return Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(stmt),
    });
  }

  /// Mints an unnamed export for an expression
  /// module.exports = 'hello'
  pub fn require_export_default(&self, stmt: Box<Expr>) -> Stmt {
    let mut template = self.export_all_require_stmt.clone();

    let expr = template.as_mut_expr().unwrap();
    let assign = expr.expr.as_mut_assign().unwrap();
    let call = assign.right.as_mut_call().unwrap();

    call.args[0].expr = stmt;

    return template;
  }

  /// Mints a named export for an expression
  /// module.exports.a = 'hello'
  pub fn require_export_named(&self, key: &str, stmt: Box<Expr>) -> Stmt {
    let mut template = self.export_all_require_stmt.clone();

    let expr = template.as_mut_expr().unwrap();
    let assign = expr.expr.as_mut_assign().unwrap();
    let call = assign.right.as_mut_call().unwrap();

    call.args[0].expr = stmt;

    call.args.push(ExprOrSpread {
      spread: None,
      expr: Box::new(Expr::Lit(Lit::Str(Str {
        span: Span::default(),
        value: Atom::from(key),
        raw: Some(Atom::from(format!("\"{}\"", key))),
      }))),
    });

    return template;
  }

  /// Mints an export statement that reexports all exports from a target module
  /// export * from './foo'
  pub fn reexport_all(&self, specifier: &str) -> Stmt {
    let mut stmt = self.export_all_stmt.clone();

    let Stmt::Expr(expr) = &mut stmt else {
      panic!("Unable to generate import");
    };

    let Expr::Call(expr) = expr.expr.as_mut() else {
      panic!("Unable to generate import");
    };

    let arg = &mut expr.args[0];

    arg.expr = Box::new(Expr::Lit(Lit::Str(Str {
      span: Span::default(),
      value: Atom::from(specifier),
      raw: Some(Atom::from(format!("\"{}\"", specifier))),
    })));

    return stmt;
  }

  /// Mints an export statement that reexports all exports from a target module under a namespace
  /// export * as bar from './foo'
  pub fn reexport_all_rename(&self, specifier: &str, rename: &str) -> Stmt {
    let import_expr = self.import(specifier);

    let Stmt::Expr(import_expr) = import_expr else {
      panic!("Unable to generate import");
    };

    let Expr::Call(import_expr) = import_expr.expr.as_ref() else {
      panic!("Unable to generate import");
    };

    let stmt = Expr::Assign(AssignExpr {
      span: Span::default(),
      op: AssignOp::Assign,
      left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
        span: Span::default(),
        obj: Box::new(Expr::Ident(Ident {
          span: Span::default(),
          sym: Atom::from(RUNTIME_EXPORT_SYMBOL),
          optional: false,
        })),
        prop: MemberProp::Computed(ComputedPropName {
          span: Span::default(),
          expr: Box::new(Expr::Lit(Lit::Str(Str {
            span: Span::default(),
            value: Atom::from(rename),
            raw: Some(Atom::from(format!("\"{}\"", rename))),
          }))),
        }),
      }))),
      right: Box::new(Expr::Call(import_expr.clone())),
    });

    return Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(stmt),
    });
  }

  /// Mints an export statement for a single property from a target module
  /// export { bar } from './foo'
  pub fn reexport(&self, specifier: &str, import_key: &str) -> Stmt {
    return self.reexport_rename(specifier, import_key, import_key);
  }

  /// Mints an export statement for a single property from a target module, renaming the export
  /// export { foo as bar } from './foo'
  pub fn reexport_rename(&self, specifier: &str, import_key: &str, export_as: &str) -> Stmt {
    let import_expr = self.import(specifier);

    let Stmt::Expr(import_expr) = import_expr else {
      panic!("Unable to generate import");
    };

    let Expr::Call(import_expr) = import_expr.expr.as_ref() else {
      panic!("Unable to generate import");
    };

    let stmt = Expr::Assign(AssignExpr {
      span: Span::default(),
      op: AssignOp::Assign,
      left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
        span: Span::default(),
        obj: Box::new(Expr::Ident(Ident {
          span: Span::default(),
          sym: Atom::from(RUNTIME_EXPORT_SYMBOL),
          optional: false,
        })),
        prop: MemberProp::Computed(ComputedPropName {
          span: Span::default(),
          expr: Box::new(Expr::Lit(Lit::Str(Str {
            span: Span::default(),
            value: Atom::from(export_as),
            raw: Some(Atom::from(format!("\"{}\"", export_as))),
          }))),
        }),
      }))),
      right: Box::new(Expr::Member(MemberExpr {
        span: Span::default(),
        obj: Box::new(Expr::Call(import_expr.clone())),
        prop: MemberProp::Computed(ComputedPropName {
          span: Span::default(),
          expr: Box::new(Expr::Lit(Lit::Str(Str {
            span: Span::default(),
            value: Atom::from(import_key),
            raw: Some(Atom::from(format!("\"{}\"", import_key))),
          }))),
        }),
      })),
    });

    return Stmt::Expr(ExprStmt {
      span: Span::default(),
      expr: Box::new(stmt),
    });
  }
}

/// Describes the variable assignments of an import
pub enum ImportNamed {
  /// import { foo } from 'specifier'
  ///          ---
  Key(String),
  /// import { foo as bar } from 'specifier'
  ///          ----------
  Renamed(String, String),
  /// import foo from 'specifier'
  ///        ---
  Default(String),
}
