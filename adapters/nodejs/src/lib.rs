mod nodejs_adapter;
mod transformer;
mod resolver;
mod engine;

use engine::NodejsConnection;
use libmach::Adapter;
use libmach::AdapterBootstrapResult;
use libmach::AdapterBootstrapOptions;
use libmach::AdapterOption;
use std::sync::Arc;

#[no_mangle]
pub extern fn bootstrap(options: AdapterBootstrapOptions) -> AdapterBootstrapResult {
  return Box::new(Box::pin(async move {
    let AdapterOption::Usize(node_workers) = &options.get("node_workers").unwrap_or(&AdapterOption::Usize(1)) else {
      return Err("Invalid type for 'node_workers'".to_string());
    };

    let nodejs_connection = Arc::new(NodejsConnection::new(node_workers.clone()).await);

    let adapter: Box<dyn Adapter> = Box::new(nodejs_adapter::NodejsAdapter{
      nodejs_connection,
    });

    return Ok(adapter);
  }));
}
