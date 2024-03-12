// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use crate::lsp::analysis::source_range_to_lsp_range;

use super::definitions::TestModule;

use deno_ast::swc::ast;
use deno_ast::swc::visit::Visit;
use deno_ast::swc::visit::VisitWith;
use deno_ast::SourceRangedForSpanned;
use deno_ast::SourceTextInfo;
use deno_core::ModuleSpecifier;
use lsp::Range;
use std::collections::HashMap;
use std::collections::HashSet;
use tower_lsp::lsp_types as lsp;

/// Parse an arrow expression for any test steps and return them.
fn visit_arrow(
  arrow_expr: &ast::ArrowExpr,
  parent_id: &str,
  text_info: &SourceTextInfo,
  test_module: &mut TestModule,
) {
  if let Some((maybe_test_context, maybe_step_var)) =
    parse_test_context_param(arrow_expr.params.first())
  {
    let mut collector = TestStepCollector::new(
      maybe_test_context,
      maybe_step_var,
      parent_id,
      text_info,
      test_module,
    );
    arrow_expr.body.visit_with(&mut collector);
  }
}

/// Parse a function for any test steps and return them.
fn visit_fn(
  function: &ast::Function,
  parent_id: &str,
  text_info: &SourceTextInfo,
  test_module: &mut TestModule,
) {
  if let Some((maybe_test_context, maybe_step_var)) =
    parse_test_context_param(function.params.first().map(|p| &p.pat))
  {
    let mut collector = TestStepCollector::new(
      maybe_test_context,
      maybe_step_var,
      parent_id,
      text_info,
      test_module,
    );
    function.body.visit_with(&mut collector);
  }
}

/// Parse a param of a test function for the test context binding, or any
/// destructuring of a `steps` method from the test context.
fn parse_test_context_param(
  param: Option<&ast::Pat>,
) -> Option<(Option<String>, Option<String>)> {
  let mut maybe_test_context = None;
  let mut maybe_step_var = None;
  match param {
    // handles `(testContext)`
    Some(ast::Pat::Ident(binding_ident)) => {
      maybe_test_context = Some(binding_ident.id.sym.to_string());
    }
    Some(ast::Pat::Object(object_pattern)) => {
      for prop in &object_pattern.props {
        match prop {
          ast::ObjectPatProp::KeyValue(key_value_pat_prop) => {
            match &key_value_pat_prop.key {
              // handles `({ step: s })`
              ast::PropName::Ident(ident) => {
                if ident.sym.eq("step") {
                  if let ast::Pat::Ident(ident) =
                    key_value_pat_prop.value.as_ref()
                  {
                    maybe_step_var = Some(ident.id.sym.to_string());
                  }
                  break;
                }
              }
              // handles `({ "step": s })`
              ast::PropName::Str(string) => {
                if string.value.eq("step") {
                  if let ast::Pat::Ident(ident) =
                    key_value_pat_prop.value.as_ref()
                  {
                    maybe_step_var = Some(ident.id.sym.to_string());
                  }
                  break;
                }
              }
              _ => (),
            }
          }
          // handles `({ step = something })`
          ast::ObjectPatProp::Assign(assign_pat_prop)
            if assign_pat_prop.key.sym.eq("step") =>
          {
            maybe_step_var = Some("step".to_string());
            break;
          }
          // handles `({ ...ctx })`
          ast::ObjectPatProp::Rest(rest_pat) => {
            if let ast::Pat::Ident(ident) = rest_pat.arg.as_ref() {
              maybe_test_context = Some(ident.id.sym.to_string());
            }
            break;
          }
          _ => (),
        }
      }
    }
    _ => return None,
  }
  if maybe_test_context.is_none() && maybe_step_var.is_none() {
    None
  } else {
    Some((maybe_test_context, maybe_step_var))
  }
}

/// Check a call expression of a test or test step to determine the name of the
/// test or test step as well as any sub steps.
fn visit_call_expr(
  node: &ast::CallExpr,
  fns: Option<&HashMap<String, ast::Function>>,
  range: Range,
  parent_id: Option<&str>,
  text_info: &SourceTextInfo,
  test_module: &mut TestModule,
) {
  if let Some(expr) = node.args.first().map(|es| es.expr.as_ref()) {
    match expr {
      ast::Expr::Object(obj_lit) => {
        let mut maybe_name = None;
        for prop in &obj_lit.props {
          let ast::PropOrSpread::Prop(prop) = prop else {
            continue;
          };
          let ast::Prop::KeyValue(key_value_prop) = prop.as_ref() else {
            continue;
          };
          let ast::PropName::Ident(ast::Ident { sym, .. }) =
            &key_value_prop.key
          else {
            continue;
          };
          if sym == "name" {
            match key_value_prop.value.as_ref() {
              // matches string literals (e.g. "test name" or
              // 'test name')
              ast::Expr::Lit(ast::Lit::Str(lit_str)) => {
                maybe_name = Some(lit_str.value.to_string());
              }
              // matches template literals with only a single quasis
              // (e.g. `test name`)
              ast::Expr::Tpl(tpl) => {
                if tpl.quasis.len() == 1 {
                  maybe_name = Some(tpl.quasis[0].raw.to_string());
                }
              }
              _ => {}
            }
            break;
          }
        }
        let name = match maybe_name {
          Some(n) => n,
          None => return,
        };
        let (id, _) = test_module.register(
          name,
          Some(range),
          false,
          parent_id.map(str::to_owned),
        );
        for prop in &obj_lit.props {
          let ast::PropOrSpread::Prop(prop) = prop else {
            continue;
          };
          match prop.as_ref() {
            ast::Prop::KeyValue(key_value_prop) => {
              let ast::PropName::Ident(ast::Ident { sym, .. }) =
                &key_value_prop.key
              else {
                continue;
              };
              if sym == "fn" {
                match key_value_prop.value.as_ref() {
                  ast::Expr::Arrow(arrow_expr) => {
                    visit_arrow(arrow_expr, &id, text_info, test_module);
                  }
                  ast::Expr::Fn(fn_expr) => {
                    visit_fn(&fn_expr.function, &id, text_info, test_module);
                  }
                  _ => {}
                }
                break;
              }
            }
            ast::Prop::Method(method_prop) => {
              let ast::PropName::Ident(ast::Ident { sym, .. }) =
                &method_prop.key
              else {
                continue;
              };
              if sym == "fn" {
                visit_fn(&method_prop.function, &id, text_info, test_module);
                break;
              }
            }
            _ => {}
          }
        }
      }
      ast::Expr::Fn(fn_expr) => {
        if let Some(ast::Ident { sym, .. }) = fn_expr.ident.as_ref() {
          let name = sym.to_string();
          let (id, _) = test_module.register(
            name,
            Some(range),
            false,
            parent_id.map(str::to_owned),
          );
          visit_fn(&fn_expr.function, &id, text_info, test_module);
        }
      }
      ast::Expr::Lit(ast::Lit::Str(lit_str)) => {
        let name = lit_str.value.to_string();
        let (id, _) = test_module.register(
          name,
          Some(range),
          false,
          parent_id.map(str::to_owned),
        );
        match node.args.get(1).map(|es| es.expr.as_ref()) {
          Some(ast::Expr::Fn(fn_expr)) => {
            visit_fn(&fn_expr.function, &id, text_info, test_module);
          }
          Some(ast::Expr::Arrow(arrow_expr)) => {
            visit_arrow(arrow_expr, &id, text_info, test_module);
          }
          _ => {}
        }
      }
      ast::Expr::Tpl(tpl) => {
        if tpl.quasis.len() == 1 {
          let name = tpl.quasis[0].raw.to_string();
          let (id, _) = test_module.register(
            name,
            Some(range),
            false,
            parent_id.map(str::to_owned),
          );
          match node.args.get(1).map(|es| es.expr.as_ref()) {
            Some(ast::Expr::Fn(fn_expr)) => {
              visit_fn(&fn_expr.function, &id, text_info, test_module);
            }
            Some(ast::Expr::Arrow(arrow_expr)) => {
              visit_arrow(arrow_expr, &id, text_info, test_module);
            }
            _ => {}
          }
        }
      }
      ast::Expr::Ident(ident) => {
        let name = ident.sym.to_string();
        if let Some(fn_expr) = fns.and_then(|fns| fns.get(&name)) {
          let (parent_id, _) = test_module.register(
            name,
            Some(range),
            false,
            parent_id.map(str::to_owned),
          );
          visit_fn(fn_expr, &parent_id, text_info, test_module);
        }
      }
      _ => {
        if parent_id.is_none() {
          let node_range = node.range();
          let indexes = text_info.line_and_column_display(node_range.start);
          test_module.register(
            format!("Test {}:{}", indexes.line_number, indexes.column_number),
            Some(range),
            false,
            None,
          );
        }
      }
    }
  }
}

/// A structure which can be used to walk a branch of AST determining if the
/// branch contains any testing steps.
struct TestStepCollector<'a> {
  maybe_test_context: Option<String>,
  vars: HashSet<String>,
  parent_id: &'a str,
  text_info: &'a SourceTextInfo,
  test_module: &'a mut TestModule,
}

impl<'a> TestStepCollector<'a> {
  fn new(
    maybe_test_context: Option<String>,
    maybe_step_var: Option<String>,
    parent_id: &'a str,
    text_info: &'a SourceTextInfo,
    test_module: &'a mut TestModule,
  ) -> Self {
    let mut vars = HashSet::new();
    if let Some(var) = maybe_step_var {
      vars.insert(var);
    }
    Self {
      maybe_test_context,
      vars,
      parent_id,
      text_info,
      test_module,
    }
  }
}

impl Visit for TestStepCollector<'_> {
  fn visit_call_expr(&mut self, node: &ast::CallExpr) {
    if let ast::Callee::Expr(callee_expr) = &node.callee {
      match callee_expr.as_ref() {
        // Identify calls to identified variables
        ast::Expr::Ident(ident) => {
          if self.vars.contains(&ident.sym.to_string()) {
            visit_call_expr(
              node,
              None,
              source_range_to_lsp_range(&ident.range(), self.text_info),
              Some(self.parent_id),
              self.text_info,
              self.test_module,
            );
          }
        }
        // Identify calls to `test.step()`
        ast::Expr::Member(member_expr) => {
          if let Some(test_context) = &self.maybe_test_context {
            if let ast::MemberProp::Ident(ns_prop_ident) = &member_expr.prop {
              if ns_prop_ident.sym.eq("step") {
                if let ast::Expr::Ident(ident) = member_expr.obj.as_ref() {
                  if ident.sym == *test_context {
                    visit_call_expr(
                      node,
                      None,
                      source_range_to_lsp_range(
                        &ns_prop_ident.range(),
                        self.text_info,
                      ),
                      Some(self.parent_id),
                      self.text_info,
                      self.test_module,
                    );
                  }
                }
              }
            }
          }
        }
        _ => (),
      }
    }
  }

  fn visit_var_decl(&mut self, node: &ast::VarDecl) {
    if let Some(test_context) = &self.maybe_test_context {
      for decl in &node.decls {
        let Some(init) = &decl.init else {
          continue;
        };

        match init.as_ref() {
          // Identify destructured assignments of `step` from test context
          ast::Expr::Ident(ident) => {
            if ident.sym != *test_context {
              continue;
            }
            let ast::Pat::Object(object_pat) = &decl.name else {
              continue;
            };

            for prop in &object_pat.props {
              match prop {
                ast::ObjectPatProp::Assign(prop) => {
                  if prop.key.sym.eq("step") {
                    self.vars.insert(prop.key.sym.to_string());
                  }
                }
                ast::ObjectPatProp::KeyValue(prop) => {
                  if let ast::PropName::Ident(key_ident) = &prop.key {
                    if key_ident.sym.eq("step") {
                      if let ast::Pat::Ident(value_ident) = &prop.value.as_ref()
                      {
                        self.vars.insert(value_ident.id.sym.to_string());
                      }
                    }
                  }
                }
                _ => (),
              }
            }
          }
          // Identify variable assignments where the init is test context
          // `.step`
          ast::Expr::Member(member_expr) => {
            let ast::Expr::Ident(obj_ident) = member_expr.obj.as_ref() else {
              continue;
            };

            if obj_ident.sym != *test_context {
              continue;
            }

            let ast::MemberProp::Ident(prop_ident) = &member_expr.prop else {
              continue;
            };

            if prop_ident.sym.eq("step") {
              if let ast::Pat::Ident(binding_ident) = &decl.name {
                self.vars.insert(binding_ident.id.sym.to_string());
              }
            }
          }
          _ => (),
        }
      }
    }
  }
}

/// Walk an AST and determine if it contains any `Deno.test` tests.
pub struct TestCollector {
  test_module: TestModule,
  vars: HashSet<String>,
  fns: HashMap<String, ast::Function>,
  text_info: SourceTextInfo,
}

impl TestCollector {
  pub fn new(
    specifier: ModuleSpecifier,
    script_version: String,
    text_info: SourceTextInfo,
  ) -> Self {
    Self {
      test_module: TestModule::new(specifier, script_version),
      vars: HashSet::new(),
      fns: HashMap::new(),
      text_info,
    }
  }

  /// Move out the test definitions
  pub fn take(self) -> TestModule {
    self.test_module
  }
}

impl Visit for TestCollector {
  fn visit_call_expr(&mut self, node: &ast::CallExpr) {
    fn visit_if_deno_test(
      collector: &mut TestCollector,
      node: &ast::CallExpr,
      range: &deno_ast::SourceRange,
      ns_prop_ident: &ast::Ident,
      member_expr: &ast::MemberExpr,
    ) {
      if ns_prop_ident.sym == "test" {
        let ast::Expr::Ident(ident) = member_expr.obj.as_ref() else {
          return;
        };

        if ident.sym != "Deno" {
          return;
        }

        visit_call_expr(
          node,
          Some(&collector.fns),
          source_range_to_lsp_range(range, &collector.text_info),
          None,
          &collector.text_info,
          &mut collector.test_module,
        );
      }
    }

    let ast::Callee::Expr(callee_expr) = &node.callee else {
      return;
    };

    match callee_expr.as_ref() {
      ast::Expr::Ident(ident) => {
        if self.vars.contains(&ident.sym.to_string()) {
          visit_call_expr(
            node,
            Some(&self.fns),
            source_range_to_lsp_range(&ident.range(), &self.text_info),
            None,
            &self.text_info,
            &mut self.test_module,
          );
        }
      }
      ast::Expr::Member(member_expr) => {
        let ast::MemberProp::Ident(ns_prop_ident) = &member_expr.prop else {
          return;
        };

        let ns_prop_ident_name = ns_prop_ident.sym.to_string();

        visit_if_deno_test(
          self,
          node,
          &ns_prop_ident.range(),
          ns_prop_ident,
          member_expr,
        );

        if ns_prop_ident_name == "ignore" || ns_prop_ident_name == "only" {
          let ast::Expr::Member(child_member_expr) = member_expr.obj.as_ref()
          else {
            return;
          };

          let ast::MemberProp::Ident(child_ns_prop_ident) =
            &child_member_expr.prop
          else {
            return;
          };

          visit_if_deno_test(
            self,
            node,
            &ns_prop_ident.range(),
            child_ns_prop_ident,
            child_member_expr,
          );
        }
      }
      _ => (),
    }
  }

  fn visit_var_decl(&mut self, node: &ast::VarDecl) {
    for decl in &node.decls {
      let Some(init) = &decl.init else { continue };

      match init.as_ref() {
        // Identify destructured assignments of `test` from `Deno`
        ast::Expr::Ident(ident) => {
          if ident.sym != "Deno" {
            continue;
          }

          let ast::Pat::Object(object_pat) = &decl.name else {
            continue;
          };

          for prop in &object_pat.props {
            match prop {
              ast::ObjectPatProp::Assign(prop) => {
                let name = prop.key.sym.to_string();
                if name == "test" {
                  self.vars.insert(name);
                }
              }
              ast::ObjectPatProp::KeyValue(prop) => {
                let ast::PropName::Ident(key_ident) = &prop.key else {
                  continue;
                };

                if key_ident.sym == "test" {
                  if let ast::Pat::Ident(value_ident) = &prop.value.as_ref() {
                    self.vars.insert(value_ident.id.sym.to_string());
                  }
                }
              }
              _ => (),
            }
          }
        }
        // Identify variable assignments where the init is `Deno.test`
        ast::Expr::Member(member_expr) => {
          let ast::Expr::Ident(obj_ident) = member_expr.obj.as_ref() else {
            continue;
          };

          if obj_ident.sym != "Deno" {
            continue;
          };

          let ast::MemberProp::Ident(prop_ident) = &member_expr.prop else {
            continue;
          };

          if prop_ident.sym != "test" {
            continue;
          }

          if let ast::Pat::Ident(binding_ident) = &decl.name {
            self.vars.insert(binding_ident.id.sym.to_string());
          }
        }
        _ => (),
      }
    }
  }

  fn visit_fn_decl(&mut self, n: &ast::FnDecl) {
    self
      .fns
      .insert(n.ident.sym.to_string(), *n.function.clone());
  }
}
