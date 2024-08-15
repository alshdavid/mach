use petgraph::visit::NodeRef;
use petgraph::visit::EdgeRef;

use crate::types::Compilation;

pub fn bundle_single(
  c: &mut Compilation,
) -> anyhow::Result<()> {
  println!("hi");
  let asset_graph = c.asset_graph.as_graph();
  let root_node = c.asset_graph.root_node();
  
  let n = asset_graph.neighbors(root_node);
  for node_index in n { 
    let asset = c.asset_graph.get_asset(node_index).unwrap();

    println!("fp {:?}", asset.file_path_relative);
    // let mut edges = c.asset_graph.get_dependencies(&node_index);
    // while let Some(edge) = edges.next() {
    //   let dependency = edge.weight();
    //   let dest_asset = c.asset_graph.get_asset(edge.target().id()).unwrap();
    // }
  }

  return Ok(());
}
