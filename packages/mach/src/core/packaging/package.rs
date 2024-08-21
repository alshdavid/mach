use crate::types::Compilation;

pub fn package(Compilation { 
  bundle_graph,
  asset_graph,
  .. 
}: &mut Compilation) -> anyhow::Result<()> {
  // let mut queue = vec![];

  // while let Some(bundle) = queue.pop() {}

  return Ok(());
}
