use adapter::NoopAdapter;
use libmach::Adapter;
use libmach::AdapterBootstrapOptions;
use libmach::AdapterBootstrapResult;

mod adapter;
mod resolver;
mod transformer;

#[no_mangle]
pub extern "C" fn bootstrap(_config: AdapterBootstrapOptions) -> AdapterBootstrapResult {
  let adapter: Box<dyn Adapter> = Box::new(NoopAdapter {});
  return Box::new(Box::new(Ok(adapter)));
}
