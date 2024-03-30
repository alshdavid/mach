use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use super::Resolver;
use super::Transformer;

#[derive(Debug)]
pub enum AdapterOption {
  String(String),
  PathBuf(PathBuf),
  Usize(usize),
  Bool(bool),
  HashMap(HashMap<String, AdapterOption>),
  Vec(Vec<AdapterOption>),
}

pub type AdapterOptions = HashMap<String, AdapterOption>;

pub trait Adapter: Send {
  fn get_transformer(&self, transformer_config: AdapterOptions) -> Result<Box<dyn Transformer>, String>;
  fn get_resolver(&self, resolver_config: AdapterOptions) -> Result<Box<dyn Resolver>, String>;
  fn resolve_specifier(&self, from_path: &Path, specifier: &str) -> Result<PathBuf, String>;
}

pub type AdapterBootstrapResult = Box<Box<Result<Box<dyn Adapter>, String>>>;
pub type AdapterBootstrapOptions = Box<AdapterOptions>;
pub type AdapterBootstrapFn = fn(AdapterBootstrapOptions) -> AdapterBootstrapResult;

pub type AdapterMap = HashMap<String, Box<dyn Adapter>>;
