use std::cell::OnceCell;

use anyhow::Context;

#[derive(Clone, Debug, Default)]
pub struct Identifier<T>(OnceCell<T>);

impl<T: Clone> Identifier<T> {
  pub fn set(
    &mut self,
    value: T,
  ) -> anyhow::Result<()> {
    self
      .0
      .set(value)
      .map_err(|_| anyhow::anyhow!("ID already set"))
  }

  pub fn get(&self) -> anyhow::Result<&T> {
    self.0.get().context("Value not set")
  }
}
