use is_macro::Is;
use swc_common::ast_node;
use swc_common::util::take::Take;
use swc_common::EqIgnoreSpan;
use swc_common::Span;
use swc_common::DUMMY_SP;

use crate::class::Decorator;
use crate::pat::Pat;
use crate::stmt::BlockStmt;
use crate::typescript::TsParamProp;
use crate::typescript::TsTypeAnn;
use crate::typescript::TsTypeParamDecl;

/// Common parts of function and method.
#[ast_node]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Function {
  pub params: Vec<Param>,

  #[cfg_attr(feature = "serde-impl", serde(default))]
  pub decorators: Vec<Decorator>,

  pub span: Span,

  #[cfg_attr(feature = "serde-impl", serde(default))]
  pub body: Option<BlockStmt>,

  /// if it's a generator.
  #[cfg_attr(feature = "serde-impl", serde(default, rename = "generator"))]
  pub is_generator: bool,

  /// if it's an async function.
  #[cfg_attr(feature = "serde-impl", serde(default, rename = "async"))]
  pub is_async: bool,

  #[cfg_attr(feature = "serde-impl", serde(default, rename = "typeParameters"))]
  pub type_params: Option<Box<TsTypeParamDecl>>,

  #[cfg_attr(feature = "serde-impl", serde(default))]
  pub return_type: Option<Box<TsTypeAnn>>,
}

impl Take for Function {
  fn dummy() -> Self {
    Function {
      params: Take::dummy(),
      decorators: Take::dummy(),
      span: DUMMY_SP,
      body: Take::dummy(),
      is_generator: false,
      is_async: false,
      type_params: Take::dummy(),
      return_type: Take::dummy(),
    }
  }
}

#[ast_node("Parameter")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Param {
  pub span: Span,
  #[cfg_attr(feature = "serde-impl", serde(default))]
  pub decorators: Vec<Decorator>,
  pub pat: Pat,
}

impl From<Pat> for Param {
  fn from(pat: Pat) -> Self {
    Self {
      span: DUMMY_SP,
      decorators: Default::default(),
      pat,
    }
  }
}

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum ParamOrTsParamProp {
  #[tag("TsParameterProperty")]
  TsParamProp(TsParamProp),
  #[tag("Parameter")]
  Param(Param),
}
