#![allow(clippy::let_unit_value)]
#![deny(non_snake_case)]

use std::ops::Deref;
use std::ops::DerefMut;

use swc_atoms::Atom;
use swc_atoms::JsWord;
use swc_common::collections::AHashMap;
use swc_common::comments::Comments;
use swc_common::input::StringInput;
use swc_common::BytePos;
use swc_common::Span;
use swc_ecma_ast::*;

use self::input::Buffer;
pub use self::input::Capturing;
pub use self::input::Tokens;
pub use self::input::TokensInput;
use self::util::ParseObject;
use crate::error::SyntaxError;
use crate::lexer::Lexer;
use crate::token::Token;
use crate::token::Word;
use crate::Context;
use crate::EsVersion;
use crate::Syntax;
use crate::TsConfig;

use crate::error::Error;

#[macro_use]
mod macros;
mod class_and_fn;
mod expr;
mod ident;
pub mod input;
mod jsx;
mod object;
mod pat;
mod stmt;
#[cfg(test)]
mod tests;
#[cfg(feature = "typescript")]
mod typescript;
mod util;

/// When error occurs, error is emitted and parser returns Err(()).
pub type PResult<T> = Result<T, Error>;

/// EcmaScript parser.
#[derive(Clone)]
pub struct Parser<I: Tokens> {
  state: State,
  input: Buffer<I>,
}

#[derive(Clone, Default)]
struct State {
  labels: Vec<JsWord>,
  /// Start position of an assignment expression.
  potential_arrow_start: Option<BytePos>,

  found_module_item: bool,
  /// Start position of an AST node and the span of its trailing comma.
  trailing_commas: AHashMap<BytePos, Span>,
}

impl<'a> Parser<Lexer<'a>> {
  pub fn new(
    syntax: Syntax,
    input: StringInput<'a>,
    comments: Option<&'a dyn Comments>,
  ) -> Self {
    Self::new_from(Lexer::new(syntax, Default::default(), input, comments))
  }
}

impl<I: Tokens> Parser<I> {
  pub fn new_from(mut input: I) -> Self {
    #[cfg(feature = "typescript")]
    let in_declare = matches!(
      input.syntax(),
      Syntax::Typescript(TsConfig { dts: true, .. })
    );
    #[cfg(not(feature = "typescript"))]
    let in_declare = false;
    let ctx = Context {
      in_declare,
      ..input.ctx()
    };
    input.set_ctx(ctx);

    Parser {
      state: Default::default(),
      input: Buffer::new(input),
    }
  }

  pub fn take_errors(&mut self) -> Vec<Error> {
    self.input().take_errors()
  }

  pub fn parse_script(&mut self) -> PResult<Script> {
    trace_cur!(self, parse_script);

    let ctx = Context {
      module: false,
      ..self.ctx()
    };
    self.set_ctx(ctx);

    let start = cur_pos!(self);

    let shebang = self.parse_shebang()?;

    self.parse_block_body(true, true, None).map(|body| Script {
      span: span!(self, start),
      body,
      shebang,
    })
  }

  pub fn parse_typescript_module(&mut self) -> PResult<Module> {
    trace_cur!(self, parse_typescript_module);

    debug_assert!(self.syntax().typescript());

    //TODO: parse() -> PResult<Program>
    let ctx = Context {
      module: true,
      strict: false,
      ..self.ctx()
    };
    // Module code is always in strict mode
    self.set_ctx(ctx);

    let start = cur_pos!(self);
    let shebang = self.parse_shebang()?;

    self.parse_block_body(true, true, None).map(|body| Module {
      span: span!(self, start),
      body,
      shebang,
    })
  }

  /// Returns [Module] if it's a module and returns [Script] if it's not a
  /// module.
  ///
  /// Note: This is not perfect yet. It means, some strict mode violations may
  /// not be reported even if the method returns [Module].
  pub fn parse_program(&mut self) -> PResult<Program> {
    let start = cur_pos!(self);
    let shebang = self.parse_shebang()?;
    let ctx = Context {
      can_be_module: true,
      ..self.ctx()
    };

    let body: Vec<ModuleItem> = self.with_ctx(ctx).parse_block_body(true, true, None)?;
    let has_module_item = self.state.found_module_item
      || body
        .iter()
        .any(|item| matches!(item, ModuleItem::ModuleDecl(..)));
    if has_module_item && !self.ctx().module {
      let ctx = Context {
        module: true,
        can_be_module: true,
        strict: true,
        ..self.ctx()
      };
      // Emit buffered strict mode / module code violations
      self.input.set_ctx(ctx);
    }

    Ok(if has_module_item {
      Program::Module(Module {
        span: span!(self, start),
        body,
        shebang,
      })
    } else {
      let body = body
        .into_iter()
        .map(|item| match item {
          ModuleItem::ModuleDecl(_) => unreachable!("Module is handled above"),
          ModuleItem::Stmt(stmt) => stmt,
        })
        .collect();
      Program::Script(Script {
        span: span!(self, start),
        body,
        shebang,
      })
    })
  }

  pub fn parse_module(&mut self) -> PResult<Module> {
    let ctx = Context {
      module: true,
      can_be_module: true,
      strict: true,
      ..self.ctx()
    };
    // Module code is always in strict mode
    self.set_ctx(ctx);

    let start = cur_pos!(self);
    let shebang = self.parse_shebang()?;

    self.parse_block_body(true, true, None).map(|body| Module {
      span: span!(self, start),
      body,
      shebang,
    })
  }

  fn parse_shebang(&mut self) -> PResult<Option<Atom>> {
    match cur!(self, false) {
      Ok(&Token::Shebang(..)) => match bump!(self) {
        Token::Shebang(v) => Ok(Some(v)),
        _ => unreachable!(),
      },
      _ => Ok(None),
    }
  }

  fn ctx(&self) -> Context {
    self.input.get_ctx()
  }

  #[cold]
  fn emit_err(
    &self,
    span: Span,
    error: SyntaxError,
  ) {
    if self.ctx().ignore_error || !self.syntax().early_errors() {
      return;
    }

    self.emit_error(Error::new(span, error))
  }

  #[cold]
  fn emit_error(
    &self,
    error: Error,
  ) {
    if self.ctx().ignore_error || !self.syntax().early_errors() {
      return;
    }

    self.input_ref().add_error(error);
  }

  #[cold]
  fn emit_strict_mode_err(
    &self,
    span: Span,
    error: SyntaxError,
  ) {
    if self.ctx().ignore_error {
      return;
    }
    let error = Error::new(span, error);
    self.input_ref().add_module_mode_error(error);
  }
}

#[cfg(test)]
pub fn test_parser<F, Ret>(
  s: &'static str,
  syntax: Syntax,
  f: F,
) -> Ret
where
  F: FnOnce(&mut Parser<Lexer>) -> Result<Ret, Error>,
{
  crate::with_test_sess(s, |handler, input| {
    let lexer = Lexer::new(syntax, EsVersion::Es2019, input, None);
    let mut p = Parser::new_from(lexer);
    let ret = f(&mut p);
    let mut error = false;

    for err in p.take_errors() {
      error = true;
      err.into_diagnostic(handler).emit();
    }

    let res = ret.map_err(|err| err.into_diagnostic(handler).emit())?;

    if error {
      return Err(());
    }

    Ok(res)
  })
  .unwrap_or_else(|output| panic!("test_parser(): failed to parse \n{}\n{}", s, output))
}

#[cfg(test)]
pub fn test_parser_comment<F, Ret>(
  c: &dyn Comments,
  s: &'static str,
  syntax: Syntax,
  f: F,
) -> Ret
where
  F: FnOnce(&mut Parser<Lexer>) -> Result<Ret, Error>,
{
  crate::with_test_sess(s, |handler, input| {
    let lexer = Lexer::new(syntax, EsVersion::Es2019, input, Some(&c));
    let mut p = Parser::new_from(lexer);
    let ret = f(&mut p);

    for err in p.take_errors() {
      err.into_diagnostic(handler).emit();
    }

    ret.map_err(|err| err.into_diagnostic(handler).emit())
  })
  .unwrap_or_else(|output| panic!("test_parser(): failed to parse \n{}\n{}", s, output))
}
