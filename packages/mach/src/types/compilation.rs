use super::AssetContents;
use super::AssetGraph;
use super::BundleGraph;
use super::MachConfig;
use super::Machrc;
use crate::core::plugins::PluginContainer;
use crate::rpc::RpcHosts;

#[derive(Clone, Debug)]
pub struct Compilation {
  pub rpc_hosts: RpcHosts,
  pub machrc: Machrc,
  pub config: MachConfig,
  pub asset_contents: AssetContents,
  pub asset_graph: AssetGraph,
  pub bundle_graph: BundleGraph,
  pub plugins: PluginContainer,
}
