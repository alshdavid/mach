use std::collections::HashMap;
use super::Resolver;

pub trait Adapter {
  fn bootstrap(bootstrap_config: HashMap<String, String>) -> Self;
  // fn get_transformer(&self, transformer_config: HashMap<String, String>) -> ;
  fn get_resolver(&self, resolver_config: HashMap<String, String>) -> Box<dyn Resolver>;
}
