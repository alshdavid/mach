use adapter::NoopAdapter;
use libmach::Adapter;
use libmach::AdapterBootstrapResult;
use libmach::AdapterBootstrapOptions;

mod adapter;
mod resolver;
mod transformer;

#[no_mangle]
pub extern fn bootstrap(config: AdapterBootstrapOptions) -> AdapterBootstrapResult {
  println!("ho");
  // if config.config.debug == true {
  //   dbg!(&config);
  // }
  let adapter: Box<dyn Adapter> = Box::new(NoopAdapter{});

  println!("ho");

  return Box::new(Box::new(Ok(adapter)));
}
