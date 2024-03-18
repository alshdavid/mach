use std::collections::HashMap;
use std::future::Future;
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;
use async_trait::async_trait;

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

#[async_trait]
pub trait Adapter: Send {
  async fn get_transformer(&self, transformer_config: AdapterOptions) -> Result<Box<dyn Transformer>, String>;
  async fn get_resolver(&self, resolver_config: AdapterOptions) -> Result<Box<dyn Resolver>, String>;
  async fn resolve_specifier(&self, from_path: &Path, specifier: &str) -> Result<PathBuf, String>;
}

pub type AdapterBootstrapResult = Box<Pin<Box<dyn Future<Output = Result<Box<dyn Adapter>, String>>>>>;
pub type AdapterBootstrapOptions = Box<AdapterOptions>;
pub type AdapterBootstrapFn = fn(AdapterBootstrapOptions) -> AdapterBootstrapResult;

pub type AdapterMap = HashMap<String, Box<dyn Adapter>>;
