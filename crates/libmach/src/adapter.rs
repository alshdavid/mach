use std::collections::HashMap;
use std::path::PathBuf;

use crate::MachConfig;

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

#[derive(Debug)]
pub struct AdapterOptions {
  pub config: MachConfig,
  // pub meta: HashMap<String, AdapterOption>
}

#[derive(Debug)]
pub struct AdapterGetPluginOptions {
  pub specifier: String,
  pub cwd: PathBuf,
  pub meta: HashMap<String, AdapterOption>
}

pub type AdapterMeta = HashMap<String, AdapterOption>;

pub trait Adapter: Send {
  fn get_transformer(
    &self,
    transformer_config: AdapterGetPluginOptions,
  ) -> Result<Box<dyn Transformer>, String>;

  fn get_resolver(
    &self,
    resolver_config: AdapterGetPluginOptions,
  ) -> Result<Box<dyn Resolver>, String>;
}

pub type AdapterBootstrapResult = Box<Box<Result<Box<dyn Adapter>, String>>>;
pub type AdapterBootstrapOptions = Box<AdapterOptions>;
pub type AdapterBootstrapFn = fn(AdapterBootstrapOptions) -> AdapterBootstrapResult;

pub type AdapterMap = HashMap<String, Box<dyn Adapter>>;
