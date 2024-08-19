use std::collections::HashMap;
use std::io::Write;
use std::process::Stdio;

use super::Bundle;
use super::BundleId;

use petgraph::stable_graph::EdgeIndex;
use petgraph::stable_graph::NodeIndex;
use petgraph::stable_graph::StableDiGraph;
use petgraph::visit::EdgeRef;
use petgraph::visit::NodeRef;

#[derive(Clone, Debug)]
pub struct BundleGraph {
  node_index: HashMap<BundleId, NodeIndex>,
  graph: StableDiGraph<Bundle, ()>,
}

impl Default for BundleGraph {
  fn default() -> Self {
    let mut bundle_graph = Self {
      node_index: Default::default(),
      graph: Default::default(),
    };

    let bundle = Bundle{
      id: Default::default(),
      kind: "ROOT".to_string(),
      entry_asset: Default::default(),
      assets: Default::default(),
    };

    bundle_graph.node_index.insert(bundle.id.clone(), bundle_graph.root_node());
    bundle_graph.graph.add_node(bundle);
    bundle_graph
  }
}

impl BundleGraph {
  pub fn add_bundle(
    &mut self,
    bundle: Bundle,
  ) -> NodeIndex {
    let bundle_id = bundle.id.clone();
    let node_id = self.graph.add_node(bundle);
    self.node_index.insert(bundle_id, node_id.clone());
    node_id
  }

  pub fn add_edge(
    &mut self,
    src: &NodeIndex,
    dest: &NodeIndex,
  ) -> anyhow::Result<(bool, EdgeIndex)> {
    let edge_id = self.graph.add_edge(src.clone(), dest.clone(), ());

    Ok((true, edge_id))
  }

  pub fn root_node(&self) -> NodeIndex {
    NodeIndex::from(0)
  }

  pub fn get_bundle(
    &self,
    index: NodeIndex,
  ) -> Option<&Bundle> {
    self.graph.node_weight(index)
  }

  pub fn get_index(
    &self,
    bundle: &BundleId,
  ) -> Option<&NodeIndex> {
    self.node_index.get(bundle)
  }

  pub fn as_graph(&self) -> &StableDiGraph<Bundle, ()> {
    &self.graph
  }
}

//
// Debugging
//
impl BundleGraph {
  pub fn debug_render_graph(&self) {
    let mut output = String::new();

    if self.graph.node_count() == 1 {
      println!("BundleGraph Empty");
      return
    }

    for node_index in self.graph.node_indices().into_iter() {
      let mut edges = self.graph.edges(node_index);
      let src = self.get_bundle(node_index).unwrap();
      let src = {
        if src.kind == "ROOT" {
          format!("ROOT")
        } else {
          format!("[{}] ({}) {:?} - {}", src.id.0, src.kind, src.entry_asset.as_ref().map(|v| v.0.clone()), src.assets.iter().map(|id| id.1.0.to_string()).collect::<Vec<String>>().join(","))
        }
      };

      while let Some(edge) = edges.next() {
        let dest = self.get_bundle(edge.target().id()).unwrap();
        let dest = format!("[{}] ({}) {:?} - {} ", dest.id.0, dest.kind, dest.entry_asset.as_ref().map(|v| v.0.clone()), dest.assets.iter().map(|id| id.1.0.to_string()).collect::<Vec<String>>().join(","));

        output.push_str(&format!("{} -> {}\n", src, dest));
      }
    }

    let mut command = std::process::Command::new("node");
    command.stdin(Stdio::piped());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().unwrap();

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all("const value = `\n".as_bytes()).unwrap();
    stdin.write_all(output.as_bytes()).unwrap();
    stdin.write_all("\n`".as_bytes()).unwrap();

    stdin
      .write_all(
        r#"
      const { init } = require('diagonjs')

      void async function() {
        const d = await init()
        console.log(d.translate.graphDAG(value))
      }()
    "#
        .as_bytes(),
      )
      .unwrap();
    drop(stdin);

    let output = child.wait_with_output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
  }
}
