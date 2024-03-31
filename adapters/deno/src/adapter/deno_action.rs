use std::{collections::HashMap, path::PathBuf, sync::{mpsc::Sender, Arc, Mutex}};

use libmach::Dependency;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum DenoAction {
  LoadResolver(PathBuf, oneshot::Sender<()>),
  RunResolverResolve(String, String, oneshot::Sender<()>),
}

pub type DependencyGetters = Arc<Mutex<HashMap<String, Sender<(String, Sender<String>)>>>>;
