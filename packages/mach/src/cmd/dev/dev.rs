#[derive(Debug)]
pub struct DevOptions {}

pub struct DevResult {}

pub fn dev(_options: DevOptions) -> Result<DevResult, String> {
  return Err("Not implemented yet".to_string());
}
