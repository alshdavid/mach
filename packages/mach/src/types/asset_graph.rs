use std::collections::HashMap;
use std::io::Write;
use std::process::Stdio;

use petgraph::dot::Config;
use petgraph::dot::Dot;
use petgraph::prelude::*;
use petgraph::stable_graph::EdgeIndex;
use petgraph::stable_graph::EdgeReference;
use petgraph::stable_graph::Edges;
use petgraph::stable_graph::NodeIndex;
use petgraph::stable_graph::StableDiGraph;
use petgraph::visit::NodeRef;

use super::Asset;
use super::AssetId;
use super::Dependency;
use super::DependencyId;
use crate::core::config::ROOT_ASSET;
use crate::types::DependencyPriority;

#[derive(Default, Clone, Debug)]
pub struct AssetGraph {
  node_index: HashMap<AssetId, NodeIndex>,
  edge_index: HashMap<DependencyId, EdgeIndex>,
  graph: StableDiGraph<Asset, Dependency>,
}

impl AssetGraph {
  pub fn add_asset(
    &mut self,
    asset: Asset,
  ) -> NodeIndex {
    let asset_id = asset.id.clone();
    let node_id = self.graph.add_node(asset);
    self.node_index.insert(asset_id, node_id.clone());
    node_id
  }

  pub fn add_dependency(
    &mut self,
    src: &AssetId,
    dest: &AssetId,
    dependency: Dependency,
  ) -> anyhow::Result<(bool, EdgeIndex)> {
    let Some(src_id) = self.node_index.get(&src) else {
      anyhow::bail!("Unable to find Source Asset with ID: {}", src);
    };
    let Some(dest_id) = self.node_index.get(&dest) else {
      anyhow::bail!("Unable to find Dest Asset with ID: {}", dest);
    };
    let dependency_id = dependency.id.clone();
    if let Some(edge_index) = self.edge_index.get(&dependency_id) {
      return Ok((false, edge_index.clone()));
    }
    let edge_id = self
      .graph
      .add_edge(src_id.clone(), dest_id.clone(), dependency);

    self.edge_index.insert(dependency_id, edge_id.clone());

    Ok((true, edge_id))
  }

  pub fn root_node(&self) -> NodeIndex {
    NodeIndex::from(0)
  }

  pub fn get_asset(
    &self,
    index: NodeIndex,
  ) -> Option<&Asset> {
    self.graph.node_weight(index)
  }

  pub fn get_asset_from_asset_id(
    &self,
    index: &AssetId,
  ) -> Option<&NodeIndex> {
    self.node_index.get(index)
  }

  pub fn get_dependency(
    &self,
    index: EdgeIndex,
  ) -> Option<&Dependency> {
    self.graph.edge_weight(index)
  }

  pub fn get_dependencies(
    &self,
    index: &NodeIndex,
  ) -> Edges<Dependency, Directed, u32> {
    self.graph.edges(index.clone())
  }

  pub fn as_graph(&self) -> &StableDiGraph<Asset, Dependency> {
    &self.graph
  }
}

//
// Debugging
//
impl AssetGraph {
  pub fn debug_dot(
    &self,
  ) -> String {
    let get_node_attribute = |_: &StableDiGraph<Asset, Dependency>,
                              (_, asset): (NodeIndex, &Asset)| {
      if asset.id == ROOT_ASSET.id {
        return format!("shape=box label=\"ROOT\" ");
      }

      let mut label = String::new();
      label += &format!("[{}] ", asset.id.0);
      label += &asset.file_path.to_str().unwrap().to_string();

      format!("shape=box label=\"{}\" ", label)
    };

    let get_edge_attribute =
      |_: &StableDiGraph<Asset, Dependency>, edge_ref: EdgeReference<Dependency>| -> String {
        let dependency = edge_ref.weight();
        let mut label = String::new();

        let mut specifier = dependency.specifier.clone();
        if dependency.specifier.starts_with("/") || dependency.specifier.starts_with("\\") {
          specifier = format!("");
        }

        label += &format!("label=\"  {}\" ", specifier);

        if let DependencyPriority::Lazy = dependency.priority {
          label += &format!("; style = \"dashed\" ")
        }

        label
      };

    let dot = Dot::with_attr_getters(
      &self.graph,
      &[Config::EdgeNoLabel, Config::NodeNoLabel],
      &get_edge_attribute,
      &get_node_attribute,
    );
    format!("{:?}", dot)
  }

  pub fn debug_render(&self) {
    let mut output = String::new();

    for node_index in self.graph.node_indices().into_iter() {
      let mut edges = self.get_dependencies(&node_index);
      let source_asset = self.get_asset(node_index).unwrap();
      let mut src_path = source_asset
        .file_path
        .to_str()
        .unwrap()
        .to_string();
      if src_path == "" {
        // skip root
        continue;
      } else {
        src_path = format!("[{}] {}", source_asset.id.0, src_path);
      }

      while let Some(edge) = edges.next() {
        let dest_asset = self.get_asset(edge.target().id()).unwrap();
        let dest_path = dest_asset.file_path.to_str().unwrap();
        let dest_path = format!("[{}] {}", dest_asset.id.0, dest_path);

        output.push_str(&format!("{} -> {}\n", src_path, dest_path));
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
