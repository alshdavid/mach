use crate::rpc::RpcHosts;

#[derive(Clone)]
pub struct MachOptions {
  pub rpc_hosts: RpcHosts,
  /// How many threads to use for compilation
  pub threads: usize,
}

impl Default for MachOptions {
    fn default() -> Self {
        Self { 
          rpc_hosts: Default::default(), 
          threads: num_cpus::get(), 
        }
    }
}
