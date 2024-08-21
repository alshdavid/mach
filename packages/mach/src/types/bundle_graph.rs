use anyhow::Context;
use once_cell::sync::Lazy;
use petgraph::stable_graph::EdgeIndex;
use petgraph::stable_graph::StableDiGraph;

use super::Bundle;
use super::BundleId;

pub type BundleGraph = StableDiGraph<Bundle, ()>;

pub static ROOT_BUNDLE: Lazy<BundleId> = Lazy::new(|| BundleId::new(0));

pub trait BundleGraphExt {
  fn add_bundle(
    &mut self,
    bundle: Bundle,
  ) -> &mut Bundle;
  fn add_edge(
    &mut self,
    src: BundleId,
    dest: BundleId,
  ) -> EdgeIndex;
  fn get_bundle(
    &self,
    id: BundleId,
  ) -> Option<&Bundle>;
  fn try_get_bundle(
    &self,
    id: BundleId,
  ) -> anyhow::Result<&Bundle>;
  fn get_bundle_mut(
    &mut self,
    id: BundleId,
  ) -> Option<&mut Bundle>;
  fn try_get_bundle_mut(
    &mut self,
    id: BundleId,
  ) -> anyhow::Result<&mut Bundle>;
}

impl BundleGraphExt for BundleGraph {
  fn add_bundle(
    &mut self,
    bundle: Bundle,
  ) -> &mut Bundle {
    let nx = self.add_node(bundle);
    let bundle = self.node_weight_mut(nx.clone()).unwrap();
    bundle.id.set(nx).unwrap();
    bundle
  }

  fn add_edge(
    &mut self,
    src: BundleId,
    dest: BundleId,
  ) -> EdgeIndex {
    self.add_edge(src, dest, ())
  }

  fn get_bundle(
    &self,
    id: BundleId,
  ) -> Option<&Bundle> {
    self.node_weight(id)
  }

  fn try_get_bundle(
    &self,
    id: BundleId,
  ) -> anyhow::Result<&Bundle> {
    self.get_bundle(id).context("Bundle does not exist")
  }

  fn get_bundle_mut(
    &mut self,
    id: BundleId,
  ) -> Option<&mut Bundle> {
    self.node_weight_mut(id)
  }

  fn try_get_bundle_mut(
    &mut self,
    id: BundleId,
  ) -> anyhow::Result<&mut Bundle> {
    self.get_bundle_mut(id).context("Bundle does not exist")
  }
}
