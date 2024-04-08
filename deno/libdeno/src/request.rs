use std::sync::mpsc::Sender;

#[derive(Clone)]
pub enum DenoMachRequest {
  Ping(usize, Sender<DenoPingResponse>),
  ResolverLoad(String, Sender<DenoResolverLoadResponse>),
  ResolverRun(String, String, Sender<DenoResolverRunResponse>),
}

#[derive(Clone)]
pub struct DenoPingResponse {}

#[derive(Clone)]
pub struct DenoResolverLoadResponse {}

#[derive(Clone)]
pub struct DenoResolverRunResponse {}
