use std::collections::HashSet;

use petgraph::dot::Config;
use petgraph::dot::Dot;

use crate::core::config::ROOT_ASSET;
use crate::types::DependencyPriority;

use super::Compilation;

impl Compilation {
  pub fn debug_asset_graph_dot(&self) -> String {
    let asset_graph = self.asset_graph.as_graph();
    let dot = Dot::with_attr_getters(
      &asset_graph,
      &[Config::EdgeNoLabel, Config::NodeNoLabel],
      &|_, edge_ref| -> String {
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
      },
      &|_, (_, asset)| {
        if asset.id == ROOT_ASSET.id {
          return format!("shape=box label=\"Asset Graph\" ");
        }

        let mut label = String::new();
        label += &format!("[{}] ", asset.id.0);
        label += &asset.file_path.to_str().unwrap().to_string();

        format!("shape=box label=\"{}\" ", label)
      },
    );
    format!("{:?}", dot)
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

/*

digraph G {
    graph [shape=box fontsize=10 fontname="monospace"];
    node [shape=box fontsize=10 fontname="monospace", margin="0.2,0" height=0];

    subgraph cluster_0 {
        label = "index.html";
        graph [ranksep="0.02"];
        edge[style=invis];
        0 [label="src/index.html"]
    }
    
    subgraph cluster_1 {
        label = "index.css";
        graph [ranksep="0.02"];
        edge[style=invis];
        1 [label="src/index.css"]
    }


    subgraph cluster_2 {
        label = "index.js";
        graph [ranksep="0.02"];
        edge[style=invis];
        2[label="src/index.js"]
        3[label="src/a.js"]
        2 -> 3
    }
    
    0 -> 1 [style = "dashed" color=grey]
    0 -> 2 [style = "dashed" color=grey]
}


*/