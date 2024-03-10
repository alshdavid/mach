use swc_common::chain;
use swc_common::Mark;
use swc_ecma_visit::Fold;

pub use self::async_arrows_in_class::async_arrows_in_class;
pub use self::edge_default_param::edge_default_param;
pub use self::safari_id_destructuring_collision_in_function_expression::safari_id_destructuring_collision_in_function_expression;
pub use self::template_literal_caching::template_literal_caching;

mod async_arrows_in_class;
mod edge_default_param;
mod safari_id_destructuring_collision_in_function_expression;
mod template_literal_caching;

pub fn bugfixes(unresolved_mark: Mark) -> impl Fold {
  chain!(
    async_arrows_in_class(unresolved_mark),
    edge_default_param(),
    template_literal_caching(),
    safari_id_destructuring_collision_in_function_expression()
  )
}
