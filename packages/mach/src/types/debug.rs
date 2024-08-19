use std::collections::HashSet;

use petgraph::visit::EdgeRef;
use petgraph::visit::NodeRef;

use crate::types::DependencyPriority;

use super::Compilation;

#[derive(Debug, Default)]
pub struct DebugAssetGraphOptions {
  pub show_specifiers: bool,
}

impl Compilation {
  pub fn debug_asset_graph_dot(&self, DebugAssetGraphOptions{ show_specifiers }: DebugAssetGraphOptions) -> String {
    let mut output = "digraph {\n".to_string();
    output += "graph [shape=box];\n";
    output += "node [shape=box];\n";

    let asset_graph = self.asset_graph.as_graph();
    let root_nx = self.asset_graph.root_node();

    for asset_nx in asset_graph.node_indices() {
      let asset = self.asset_graph.get_with_nx(asset_nx).unwrap();
      output += &format!("{} [label=\"", asset.id.0);

      if asset_nx == root_nx {
        output += &format!("Asset Graph\"]\n");
        continue;
      }

      output += &format!("{}\"]\n", asset.file_path.file_name().unwrap().to_str().unwrap());
    }

    let mut nodes = vec![self.bundle_graph.root_node()];
    let mut completed = HashSet::new();

    while let Some(current_nx) = nodes.pop() {
      let current_asset = self.asset_graph.get_with_nx(current_nx.clone()).unwrap();

      for next_ref in self.asset_graph.get_dependencies(&current_nx) {
        let next_nx = next_ref.target().id();
        let next_asset = self.asset_graph.get_with_nx(next_nx.clone()).unwrap();
        let dependency = next_ref.weight();

        output += &format!("{} -> {} [", current_asset.id.0, next_asset.id.0);

        if let DependencyPriority::Lazy = dependency.priority {
          output += &format!("style=\"dashed\"");
        }

        if show_specifiers && current_nx != root_nx {
          output += &format!("label=\"{}\"", dependency.specifier);
        }

        output += &format!("]\n");

        
        if !completed.contains(&next_nx) {
          nodes.push(next_nx);
        }
      }

      completed.insert(current_nx);
    }

    output += "}";
    output
  }

  pub fn debug_bundle_graph_dot(&self) -> String {
    let mut output = "digraph {\n".to_string();
    output += "graph [shape=box];\n";
    output += "node [shape=box height=0];\n";

    let bundle_graph = self.bundle_graph.as_graph();
    let root_nx = self.bundle_graph.root_node();

    for bundle_nx in bundle_graph.node_indices() {
      let bundle = self.bundle_graph.get_bundle(bundle_nx).unwrap();
      output += &format!("{} [label=\"", bundle.id.0);

      if bundle_nx == root_nx {
        output += &format!("Bundle Graph\"]\n");
        continue;
      }

      if let Some(entry) = &bundle.entry_asset {
        let entry_asset = self.asset_graph.get(entry).unwrap();
        output += &format!("{}\\l", entry_asset.file_path.file_name().unwrap().to_str().unwrap());
      } else {
        output += &format!("shared.{}.{}\\l", bundle.id.0, bundle.kind);
      };

      output += &format!("{}\\l", &bundle
        .assets
        .iter()
        .map(|id| {
          let asset = self.asset_graph.get(id.1).unwrap();
          format!("    [{}] {}", asset.id.0, asset.file_path.to_str().unwrap())
        })
        .collect::<Vec<String>>()
        .join("\\l"));

      output += &format!("\"]\n");
    }

    let mut nodes = vec![self.bundle_graph.root_node()];
    let mut completed = HashSet::new();

    while let Some(current_nx) = nodes.pop() {
      let current_bundle = self.bundle_graph.get_bundle(current_nx.clone()).unwrap();

      for next_nx in bundle_graph.neighbors(current_nx) {
        let next_bundle = self.bundle_graph.get_bundle(next_nx.clone()).unwrap();
        output += &format!("{} -> {}\n", current_bundle.id.0, next_bundle.id.0);
        
        if !completed.contains(&next_nx) {
          nodes.push(next_nx);
        }
      }

      completed.insert(current_nx);
    }

    output += "}";
    output
  }
}
