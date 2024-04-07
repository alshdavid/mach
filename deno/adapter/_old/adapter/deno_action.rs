use std::path::PathBuf;

use libmach::ResolveResult;

#[derive(Debug, Clone)]
pub enum DenoAction {
  LoadResolver(PathBuf),
  RunResolverResolve(String, String),
}

#[derive(Debug, Clone)]
pub enum DenoResponse {
  LoadResolver(()),
  RunResolverResolve(Option<ResolveResult>),
}
