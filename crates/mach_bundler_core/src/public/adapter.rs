use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

use serde::Deserialize;
use serde::Serialize;

use super::Dependency;
use super::DependencyOptions;
use super::ResolveResult;

pub trait Adapter: Debug {
  fn new(options: HashMap<String, String>) -> Self
  where
    Self: Sized;

  fn is_running(&self) -> bool;

  fn init(&self);

  fn send_all(
    &self,
    req: AdapterOutgoingRequest,
  ) -> Result<Vec<AdapterOutgoingResponse>, String>;

  fn send(
    &self,
    req: AdapterOutgoingRequest,
  ) -> Receiver<AdapterOutgoingResponse>;

  fn send_and_wait(
    &self,
    req: AdapterOutgoingRequest,
  ) -> Result<AdapterOutgoingResponse, String>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AdapterOutgoingRequest {
  Ping,
  ResolverRegister {
    specifier: String,
  },
  ResolverLoadConfig {
    specifier: String,
  },
  ResolverResolve {
    specifier: String,
    dependency: Dependency,
  },
  TransformerRegister {
    specifier: String,
  },
  TransformerLoadConfig {
    specifier: String,
  },
  TransformerTransform {
    specifier: String,
    file_path: PathBuf,
    kind: String,
    content: Vec<u8>,
  },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AdapterOutgoingResponse {
  Ping,
  ResolverRegister,
  ResolverLoadConfig,
  ResolverResolve {
    resolve_result: Option<ResolveResult>,
  },
  TransformerRegister {},
  TransformerLoadConfig {},
  TransformerTransform {
    content: Vec<u8>,
    kind: String,
    dependencies: Vec<DependencyOptions>,
  },
}

pub type Engine = String;

pub type AdapterMap = HashMap<Engine, Box<dyn Adapter>>;
