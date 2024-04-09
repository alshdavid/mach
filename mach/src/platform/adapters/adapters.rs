use std::sync::Arc;

use super::deno::DenoAdapter;

pub type AdaptersSync = Arc<Adapters>;

#[derive(Debug, Default)]
pub struct Adapters {
  pub deno: DenoAdapter
}
