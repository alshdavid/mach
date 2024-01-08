// Taken from: https://github.com/parcel-bundler/parcel/blob/v2/packages/transformers/js/core/src/env_replacer.rs

use std::collections::{HashMap, HashSet};
use std::vec;

use swc_core::common::{Mark, DUMMY_SP, Span};
use swc_core::ecma::ast;
use swc_core::ecma::atoms::{js_word, JsWord};
use swc_core::ecma::visit::{Fold, FoldWith};
use swc_core::ecma::visit::{Visit, VisitWith};

use ast::*;

pub struct NodeEnvReplacer<'a> {
  pub replace_env: bool,
  pub is_browser: bool,
  pub env: &'a HashMap<JsWord, JsWord>,
  pub decls: &'a HashSet<Id>,
  pub used_env: &'a mut HashSet<JsWord>,
  pub source_map: &'a swc_core::common::SourceMap,
  pub unresolved_mark: Mark,
}

impl<'a> Fold for NodeEnvReplacer<'a> {
  fn fold_expr(&mut self, node: Expr) -> Expr {
    // Replace assignments to process.browser with `true`
    // TODO: this seems questionable but we did it in the JS version??
    if let Expr::Assign(ref assign) = node {
      if let PatOrExpr::Pat(ref pat) = assign.left {
        if let Pat::Expr(ref expr) = &**pat {
          if let Expr::Member(ref member) = &**expr {
            if self.is_browser && match_member_expr(member, vec!["process", "browser"], self.decls)
            {
              let mut res = assign.clone();
              res.right = Box::new(Expr::Lit(Lit::Bool(Bool {
                value: true,
                span: DUMMY_SP,
              })));
              return Expr::Assign(res);
            }
          }
        }
      }
    }

    // Replace `'foo' in process.env` with a boolean.
    match &node {
      Expr::Bin(binary) if binary.op == BinaryOp::In => {
        if let (Expr::Lit(Lit::Str(left)), Expr::Member(member)) = (&*binary.left, &*binary.right) {
          if match_member_expr(member, vec!["process", "env"], self.decls) {
            return Expr::Lit(Lit::Bool(Bool {
              value: self.env.contains_key(&left.value),
              span: DUMMY_SP,
            }));
          }
        }
      }
      _ => {}
    }

    if let Expr::Member(ref member) = node {
      if self.is_browser && match_member_expr(member, vec!["process", "browser"], self.decls) {
        return Expr::Lit(Lit::Bool(Bool {
          value: true,
          span: DUMMY_SP,
        }));
      }

      if !self.replace_env {
        return node.fold_children_with(self);
      }

      if let Expr::Member(obj) = &*member.obj {
        if match_member_expr(obj, vec!["process", "env"], self.decls) {
          if let Some((sym, _)) = match_property_name(member) {
            if let Some(replacement) = self.replace(&sym, true) {
              return replacement;
            }
          }
        }
      }
    }

    if let Expr::Assign(assign) = &node {
      if !self.replace_env {
        return node.fold_children_with(self);
      }

      let expr = match &assign.left {
        PatOrExpr::Pat(pat) => {
          if let Pat::Expr(expr) = &**pat {
            Some(&**expr)
          } else if let Expr::Member(member) = &*assign.right {
            if assign.op == AssignOp::Assign
              && match_member_expr(member, vec!["process", "env"], self.decls)
            {
              let mut decls = vec![];
              self.collect_pat_bindings(pat, &mut decls);

              let mut exprs: Vec<Box<Expr>> = decls
                .iter()
                .map(|decl| {
                  Box::new(Expr::Assign(AssignExpr {
                    span: DUMMY_SP,
                    op: AssignOp::Assign,
                    left: PatOrExpr::Pat(Box::new(decl.name.clone())),
                    right: Box::new(if let Some(init) = &decl.init {
                      *init.clone()
                    } else {
                      Expr::Ident(get_undefined_ident(self.unresolved_mark))
                    }),
                  }))
                })
                .collect();

              exprs.push(Box::new(Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: vec![],
              })));

              return Expr::Seq(SeqExpr {
                span: assign.span,
                exprs,
              });
            }
            None
          } else {
            None
          }
        }
        PatOrExpr::Expr(expr) => Some(&**expr),
      };

      if let Some(Expr::Member(MemberExpr { obj, .. })) = &expr {
        if let Expr::Member(member) = &**obj {
          if match_member_expr(member, vec!["process", "env"], self.decls) {
            // self.emit_mutating_error(assign.span);
            return *assign.right.clone().fold_with(self);
          }
        }
      }
    }

    if self.replace_env {
      match &node {
        // e.g. delete process.env.SOMETHING
        Expr::Unary(UnaryExpr { op: UnaryOp::Delete, arg, span, .. }) |
        // e.g. process.env.UPDATE++
        Expr::Update(UpdateExpr { arg, span, .. }) => {
          if let Expr::Member(MemberExpr { ref obj, .. }) = &**arg {
            if let Expr::Member(member) = &**obj {
              if match_member_expr(member, vec!["process", "env"], self.decls) {
                // self.emit_mutating_error(*span);
                return match &node {
                  Expr::Unary(_) => Expr::Lit(Lit::Bool(Bool { span: *span, value: true })),
                  Expr::Update(_) => *arg.clone().fold_with(self),
                  _ => unreachable!()
                }
              }
            }
          }
        },
        _ => {}
      }
    }

    node.fold_children_with(self)
  }

  fn fold_var_decl(&mut self, node: VarDecl) -> VarDecl {
    if !self.replace_env {
      return node.fold_children_with(self);
    }

    let mut decls = vec![];
    for decl in &node.decls {
      if let Some(init) = &decl.init {
        if let Expr::Member(member) = &**init {
          if match_member_expr(member, vec!["process", "env"], self.decls) {
            self.collect_pat_bindings(&decl.name, &mut decls);
            continue;
          }
        }
      }

      decls.push(decl.clone().fold_with(self));
    }

    VarDecl {
      span: node.span,
      kind: node.kind,
      decls,
      declare: node.declare,
    }
  }
}

impl<'a> NodeEnvReplacer<'a> {
  fn replace(&mut self, sym: &JsWord, fallback_undefined: bool) -> Option<Expr> {
    if let Some(val) = self.env.get(sym) {
      self.used_env.insert(sym.clone());
      if val == "true" {
        return Some(Expr::Lit(Lit::Bool(Bool {
          span: DUMMY_SP,
          value: true,
        })));
      }
      if val == "false" {
        return Some(Expr::Lit(Lit::Bool(Bool {
          span: DUMMY_SP,
          value: false,
        })));
      }
      return Some(Expr::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value: val.clone(),
        raw: None,
      })));
    } else if fallback_undefined {
      match sym as &str {
        // don't replace process.env.hasOwnProperty with undefined
        "hasOwnProperty"
        | "isPrototypeOf"
        | "propertyIsEnumerable"
        | "toLocaleString"
        | "toSource"
        | "toString"
        | "valueOf" => {}
        _ => {
          self.used_env.insert(sym.clone());
          return Some(Expr::Ident(get_undefined_ident(self.unresolved_mark)));
        }
      };
    }
    None
  }

  fn collect_pat_bindings(&mut self, pat: &Pat, decls: &mut Vec<VarDeclarator>) {
    match pat {
      Pat::Object(object) => {
        for prop in &object.props {
          match prop {
            ObjectPatProp::KeyValue(kv) => {
              let key = match &kv.key {
                PropName::Ident(ident) => Some(ident.sym.clone()),
                PropName::Str(str) => Some(str.value.clone()),
                // Non-static. E.g. computed property.
                _ => None,
              };

              decls.push(VarDeclarator {
                span: DUMMY_SP,
                name: *kv.value.clone().fold_with(self),
                init: if let Some(key) = key {
                  self.replace(&key, false).map(Box::new)
                } else {
                  None
                },
                definite: false,
              });
            }
            ObjectPatProp::Assign(assign) => {
              // let {x} = process.env;
              // let {x = 2} = process.env;
              decls.push(VarDeclarator {
                span: DUMMY_SP,
                name: Pat::Ident(BindingIdent::from(assign.key.clone())),
                init: if let Some(init) = self.replace(&assign.key.sym, false) {
                  Some(Box::new(init))
                } else {
                  assign.value.clone().fold_with(self)
                },
                definite: false,
              })
            }
            ObjectPatProp::Rest(rest) => {
              if let Pat::Ident(ident) = &*rest.arg {
                decls.push(VarDeclarator {
                  span: DUMMY_SP,
                  name: Pat::Ident(ident.clone()),
                  init: Some(Box::new(Expr::Object(ObjectLit {
                    span: DUMMY_SP,
                    props: vec![],
                  }))),
                  definite: false,
                })
              }
            }
          }
        }
      }
      Pat::Ident(ident) => decls.push(VarDeclarator {
        span: DUMMY_SP,
        name: Pat::Ident(ident.clone()),
        init: Some(Box::new(Expr::Object(ObjectLit {
          span: DUMMY_SP,
          props: vec![],
        }))),
        definite: false,
      }),
      _ => {}
    }
  }
}

pub fn match_member_expr(expr: &ast::MemberExpr, idents: Vec<&str>, decls: &HashSet<Id>) -> bool {
  let mut member = expr;
  let mut idents = idents;
  while idents.len() > 1 {
    let expected = idents.pop().unwrap();
    let prop = match &member.prop {
      MemberProp::Computed(comp) => {
        if let Expr::Lit(Lit::Str(Str { value: ref sym, .. })) = *comp.expr {
          sym
        } else {
          return false;
        }
      }
      MemberProp::Ident(Ident { ref sym, .. }) => sym,
      _ => return false,
    };

    if prop != expected {
      return false;
    }

    match &*member.obj {
      Expr::Member(m) => member = m,
      Expr::Ident(id) => {
        return idents.len() == 1 && &id.sym == idents.pop().unwrap() && !decls.contains(&id.to_id());
      }
      _ => return false,
    }
  }

  false
}

pub fn get_undefined_ident(unresolved_mark: Mark) -> ast::Ident {
  ast::Ident::new(js_word!("undefined"), DUMMY_SP.apply_mark(unresolved_mark))
}

pub fn match_property_name(node: &ast::MemberExpr) -> Option<(JsWord, Span)> {
  match &node.prop {
    ast::MemberProp::Computed(s) => match_str(&s.expr),
    ast::MemberProp::Ident(id) => Some((id.sym.clone(), id.span)),
    ast::MemberProp::PrivateName(_) => None,
  }
}

pub fn match_str(node: &ast::Expr) -> Option<(JsWord, Span)> {
  use ast::*;

  match node {
    // "string" or 'string'
    Expr::Lit(Lit::Str(s)) => Some((s.value.clone(), s.span)),
    // `string`
    Expr::Tpl(tpl) if tpl.quasis.len() == 1 && tpl.exprs.is_empty() => {
      Some(((*tpl.quasis[0].raw).into(), tpl.span))
    }
    _ => None,
  }
}

/// This pass collects all declarations in a module into a single HashSet of tuples
/// containing identifier names and their associated syntax context (scope).
/// This is used later to determine whether an identifier references a declared variable.
/// Taken from: https://github.com/parcel-bundler/parcel/blob/v2/packages/transformers/js/core/src/decl_collector.rs
pub fn collect_decls(module: &ast::Program) -> HashSet<Id> {
  let mut c = DeclCollector {
    decls: HashSet::new(),
    in_var: false,
  };
  module.visit_with(&mut c);
  c.decls
}

struct DeclCollector {
  decls: HashSet<Id>,
  in_var: bool,
}

impl Visit for DeclCollector {
  fn visit_decl(&mut self, node: &ast::Decl) {
    use ast::Decl::*;

    match node {
      Class(class) => {
        self
          .decls
          .insert((class.ident.sym.clone(), class.ident.span.ctxt()));
      }
      Fn(f) => {
        self
          .decls
          .insert((f.ident.sym.clone(), f.ident.span.ctxt()));
      }
      _ => {}
    }

    node.visit_children_with(self);
  }

  fn visit_var_declarator(&mut self, node: &ast::VarDeclarator) {
    self.in_var = true;
    node.name.visit_with(self);
    self.in_var = false;
    if let Some(init) = &node.init {
      init.visit_with(self);
    }
  }

  fn visit_binding_ident(&mut self, node: &ast::BindingIdent) {
    if self.in_var {
      self.decls.insert((node.id.sym.clone(), node.id.span.ctxt));
    }
  }

  fn visit_assign_pat_prop(&mut self, node: &ast::AssignPatProp) {
    if self.in_var {
      self
        .decls
        .insert((node.key.sym.clone(), node.key.span.ctxt));
    }
  }

  fn visit_function(&mut self, node: &ast::Function) {
    self.in_var = true;
    for param in &node.params {
      param.visit_with(self);
    }
    self.in_var = false;

    node.body.visit_with(self);
  }

  fn visit_arrow_expr(&mut self, node: &ast::ArrowExpr) {
    self.in_var = true;
    for param in &node.params {
      param.visit_with(self);
    }
    self.in_var = false;

    node.body.visit_with(self);
  }

  fn visit_import_specifier(&mut self, node: &ast::ImportSpecifier) {
    use ast::ImportSpecifier::*;
    swc_core::ecma::visit::visit_import_specifier(self, node);

    match node {
      Default(default) => {
        self
          .decls
          .insert((default.local.sym.clone(), default.local.span.ctxt()));
      }
      Named(named) => {
        self
          .decls
          .insert((named.local.sym.clone(), named.local.span.ctxt()));
      }
      Namespace(namespace) => {
        self
          .decls
          .insert((namespace.local.sym.clone(), namespace.local.span.ctxt()));
      }
    }
  }
}
