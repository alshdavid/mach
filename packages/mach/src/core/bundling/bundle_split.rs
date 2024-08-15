use crate::types::Compilation;

pub fn bundle_split(
  _c: &mut Compilation,
) -> anyhow::Result<()> {
  println!("hey");
  anyhow::bail!("Bundle splitting is not yet enabled")
}
