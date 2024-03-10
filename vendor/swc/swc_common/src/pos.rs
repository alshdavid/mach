use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

pub use crate::syntax_pos::hygiene;
pub use crate::syntax_pos::BytePos;
pub use crate::syntax_pos::CharPos;
pub use crate::syntax_pos::FileName;
pub use crate::syntax_pos::Globals;
pub use crate::syntax_pos::Loc;
pub use crate::syntax_pos::LocWithOpt;
pub use crate::syntax_pos::Mark;
pub use crate::syntax_pos::MultiSpan;
pub use crate::syntax_pos::SourceFile;
pub use crate::syntax_pos::SourceFileAndBytePos;
pub use crate::syntax_pos::SourceFileAndLine;
pub use crate::syntax_pos::Span;
pub use crate::syntax_pos::SpanLinesError;
pub use crate::syntax_pos::SyntaxContext;
pub use crate::syntax_pos::DUMMY_SP;
pub use crate::syntax_pos::GLOBALS;
pub use crate::syntax_pos::NO_EXPANSION;

///
/// # Derive
/// This trait can be derived with `#[derive(Spanned)]`.
pub trait Spanned {
  /// Get span of `self`.
  fn span(&self) -> Span;

  #[inline]
  fn span_lo(&self) -> BytePos {
    self.span().lo
  }

  #[inline]
  fn span_hi(&self) -> BytePos {
    self.span().hi
  }
}

impl<'a, T> Spanned for Cow<'a, T>
where
  T: Spanned + Clone,
{
  #[inline]
  fn span(&self) -> Span {
    (**self).span()
  }

  #[inline]
  fn span_lo(&self) -> BytePos {
    (**self).span_lo()
  }

  #[inline]
  fn span_hi(&self) -> BytePos {
    (**self).span_hi()
  }
}

impl Spanned for Span {
  #[inline(always)]
  fn span(&self) -> Span {
    *self
  }
}

impl Spanned for BytePos {
  /// Creates a new single-byte span.
  #[inline(always)]
  fn span(&self) -> Span {
    Span::new(*self, *self, Default::default())
  }
}

impl<S> Spanned for Option<S>
where
  S: Spanned,
{
  #[inline]
  fn span(&self) -> Span {
    match *self {
      Some(ref s) => s.span(),
      None => DUMMY_SP,
    }
  }

  #[inline]
  fn span_lo(&self) -> BytePos {
    match *self {
      Some(ref s) => s.span_lo(),
      None => BytePos::DUMMY,
    }
  }

  #[inline]
  fn span_hi(&self) -> BytePos {
    match *self {
      Some(ref s) => s.span_hi(),
      None => BytePos::DUMMY,
    }
  }
}

impl<S> Spanned for Rc<S>
where
  S: ?Sized + Spanned,
{
  fn span(&self) -> Span {
    <S as Spanned>::span(self)
  }

  #[inline]
  fn span_lo(&self) -> BytePos {
    <S as Spanned>::span_lo(self)
  }

  #[inline]
  fn span_hi(&self) -> BytePos {
    <S as Spanned>::span_hi(self)
  }
}

impl<S> Spanned for Arc<S>
where
  S: ?Sized + Spanned,
{
  fn span(&self) -> Span {
    <S as Spanned>::span(self)
  }

  #[inline]
  fn span_lo(&self) -> BytePos {
    <S as Spanned>::span_lo(self)
  }

  #[inline]
  fn span_hi(&self) -> BytePos {
    <S as Spanned>::span_hi(self)
  }
}

impl<S> Spanned for Box<S>
where
  S: ?Sized + Spanned,
{
  fn span(&self) -> Span {
    <S as Spanned>::span(self)
  }

  #[inline]
  fn span_lo(&self) -> BytePos {
    <S as Spanned>::span_lo(self)
  }

  #[inline]
  fn span_hi(&self) -> BytePos {
    <S as Spanned>::span_hi(self)
  }
}

impl<'a, S> Spanned for &'a S
where
  S: ?Sized + Spanned,
{
  fn span(&self) -> Span {
    <S as Spanned>::span(self)
  }

  #[inline]
  fn span_lo(&self) -> BytePos {
    <S as Spanned>::span_lo(self)
  }

  #[inline]
  fn span_hi(&self) -> BytePos {
    <S as Spanned>::span_hi(self)
  }
}

impl<A, B> Spanned for ::either::Either<A, B>
where
  A: Spanned,
  B: Spanned,
{
  fn span(&self) -> Span {
    match *self {
      ::either::Either::Left(ref n) => n.span(),
      ::either::Either::Right(ref n) => n.span(),
    }
  }

  fn span_lo(&self) -> BytePos {
    match *self {
      ::either::Either::Left(ref n) => n.span_lo(),
      ::either::Either::Right(ref n) => n.span_lo(),
    }
  }

  fn span_hi(&self) -> BytePos {
    match *self {
      ::either::Either::Left(ref n) => n.span_hi(),
      ::either::Either::Right(ref n) => n.span_hi(),
    }
  }
}
