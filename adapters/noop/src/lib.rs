use adapter::NoopAdapter;
use libmach::Adapter;
use libmach::AdapterBootstrapResult;
use libmach::AdapterBootstrapOptions;

mod adapter;
mod resolver;
mod transformer;

#[no_mangle]
pub extern fn bootstrap(_config: AdapterBootstrapOptions) -> AdapterBootstrapResult {
  let adapter: Box<dyn Adapter> = Box::new(NoopAdapter{});
  return Box::new(Box::new(Ok(adapter)));
}
