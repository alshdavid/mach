use std::collections::HashMap;
use super::Resolver;
use super::Transformer;

pub trait Adapter: Send {
  fn get_transformer(&self, transformer_config: HashMap<String, String>) -> Box<dyn Transformer>;
  fn get_resolver(&self, resolver_config: HashMap<String, String>) -> Box<dyn Resolver>;
}

pub type AdapterBootstrap = fn(bootstrap_config: HashMap<String, String>) -> Box<dyn Adapter>;
