#![deny(clippy::all)]
#![deny(unused)]
#![allow(clippy::match_like_matches_macro)]

pub use self::const_modules::const_modules;
pub use self::debug::debug_assert_valid;
pub use self::debug::AssertValid;
pub use self::inline_globals::inline_globals;
pub use self::inline_globals::inline_globals2;
pub use self::inline_globals::GlobalExprMap;
pub use self::json_parse::json_parse;
pub use self::simplify::simplifier;

mod const_modules;
mod debug;
mod inline_globals;
mod json_parse;
pub mod simplify;
