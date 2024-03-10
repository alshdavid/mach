#![deny(clippy::all)]
#![allow(clippy::mutable_key_type)]
#![allow(clippy::arc_with_non_send_sync)]
#![allow(rustc::untranslatable_diagnostic_trivial)]

use swc_common::chain;
use swc_common::comments::Comments;
use swc_common::sync::Lrc;
use swc_common::Mark;
use swc_common::SourceMap;
use swc_ecma_visit::Fold;
use swc_ecma_visit::VisitMut;

pub use self::display_name::display_name;
pub use self::jsx::*;
pub use self::jsx_self::jsx_self;
pub use self::jsx_src::jsx_src;
pub use self::pure_annotations::pure_annotations;
pub use self::refresh::options::RefreshOptions;
pub use self::refresh::refresh;

mod display_name;
mod jsx;
mod jsx_self;
mod jsx_src;
mod pure_annotations;
mod refresh;

/// `@babel/preset-react`
///
/// Preset for all React plugins.
///
///
/// `top_level_mark` should be [Mark] passed to
/// [swc_ecma_transforms_base::resolver::resolver_with_mark].
///
///
///
/// # Note
///
/// This pass uses [swc_ecma_utils::HANDLER].
pub fn react<C>(
  cm: Lrc<SourceMap>,
  comments: Option<C>,
  mut options: Options,
  top_level_mark: Mark,
  unresolved_mark: Mark,
) -> impl Fold + VisitMut
where
  C: Comments + Clone,
{
  let Options { development, .. } = options;
  let development = development.unwrap_or(false);

  let refresh_options = options.refresh.take();

  chain!(
    jsx_src(development, cm.clone()),
    jsx_self(development),
    refresh(
      development,
      refresh_options,
      cm.clone(),
      comments.clone(),
      top_level_mark
    ),
    jsx(
      cm,
      comments.clone(),
      options,
      top_level_mark,
      unresolved_mark
    ),
    display_name(),
    pure_annotations(comments),
  )
}
