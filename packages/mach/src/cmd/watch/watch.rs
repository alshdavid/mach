#[derive(Debug)]
pub struct WatchOptions {}

pub struct WatchResult {}

pub fn watch(_options: WatchOptions) -> Result<WatchResult, String> {
  return Err("Not implemented yet".to_string());
}
