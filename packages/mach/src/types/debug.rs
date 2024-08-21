use std::collections::HashSet;

use anyhow::Context;
use petgraph::visit::EdgeRef;
use petgraph::visit::NodeRef;

use super::AssetGraphExt;
use super::BundleGraphExt;
use super::Compilation;
use super::ROOT_ASSET;
use super::ROOT_BUNDLE;
use crate::types::DependencyPriority;

#[derive(Debug, Default)]
pub struct DebugAssetGraphOptions {
  pub show_specifiers: bool,
}

impl Compilation {
  pub fn debug_asset_graph_dot(
    &self,
    DebugAssetGraphOptions { show_specifiers }: DebugAssetGraphOptions,
  ) -> anyhow::Result<String> {
    let mut output = "digraph {\n".to_string();
    output += "graph [shape=box];\n";
    output += "node [shape=box];\n";

    let root_nx = ROOT_ASSET.clone();

    for asset_id in self.asset_graph.get_assets() {
      let asset = self.asset_graph.try_get_asset(asset_id)?;
      output += &format!("{} [label=\"", asset.id.get()?.index());

      if asset_id == root_nx {
        output += &format!("Asset Graph\"]\n");
        continue;
      }

      output += &format!(
        "{}\"]\n",
        asset
          .file_path
          .file_name()
          .context("")?
          .to_str()
          .context("")?
      );
    }

    let mut nodes = vec![root_nx.clone()];
    let mut completed = HashSet::new();

    while let Some(current_nx) = nodes.pop() {
      let current_asset = self.asset_graph.try_get_asset(current_nx.clone())?;

      for next_ref in self.asset_graph.get_dependencies(current_nx) {
        let next_nx = next_ref.target().id();
        let next_asset = self.asset_graph.try_get_asset(next_nx.clone())?;
        let dependency = next_ref.weight();

        output += &format!(
          "{} -> {} [",
          current_asset.id.get()?.index(),
          next_asset.id.get()?.index()
        );

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
    Ok(output)
  }

  pub fn debug_bundle_graph_dot(&self) -> anyhow::Result<String> {
    let mut output = "digraph {\n".to_string();
    output += "graph [shape=box];\n";
    output += "node [shape=box height=0];\n";

    let root_nx = ROOT_BUNDLE.clone();

    for bundle_nx in self.bundle_graph.node_indices() {
      let bundle = self.bundle_graph.get_bundle(bundle_nx).unwrap();
      output += &format!("{} [label=\"", bundle.id.get()?.index());

      if bundle_nx == root_nx {
        output += &format!("Bundle Graph\"]\n");
        continue;
      }

      if let Some(entry) = &bundle.entry_asset {
        let entry_asset = self.asset_graph.get_asset(entry.clone()).unwrap();
        output += &format!(
          "{}\\l",
          entry_asset.file_path.file_name().unwrap().to_str().unwrap()
        );
      } else {
        output += &format!("shared.{}.{}\\l", bundle.id.get()?.index(), bundle.kind);
      };

      output += &format!(
        "{}\\l",
        &bundle
          .assets
          .iter()
          .map(|id| {
            let asset = self.asset_graph.get_asset(id.1.clone()).unwrap();
            format!(
              "    [{}] {}",
              asset.id.get().unwrap().index(),
              asset.file_path.to_str().unwrap()
            )
          })
          .collect::<Vec<String>>()
          .join("\\l")
      );

      output += &format!("\"]\n");
    }

    let mut nodes = vec![root_nx.clone()];
    let mut completed = HashSet::new();

    while let Some(current_nx) = nodes.pop() {
      let current_bundle = self.bundle_graph.get_bundle(current_nx.clone()).unwrap();

      for next_nx in self.bundle_graph.neighbors(current_nx) {
        let next_bundle = self.bundle_graph.get_bundle(next_nx.clone()).unwrap();
        output += &format!(
          "{} -> {}\n",
          current_bundle.id.get()?.index(),
          next_bundle.id.get()?.index()
        );

        if !completed.contains(&next_nx) {
          nodes.push(next_nx);
        }
      }

      completed.insert(current_nx);
    }

    output += "}";
    Ok(output)
  }
}
