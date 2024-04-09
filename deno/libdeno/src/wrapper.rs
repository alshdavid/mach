use std::sync::mpsc::channel;

use crate::DenoMachRequest;
use crate::DenoPingResponse;
use crate::DenoResolverLoadResponse;
use crate::DenoResolverRunResponse;
use crate::DenoWorkerFarm;

#[derive(Debug, Default)]
pub struct DenoWrapper {
  worker_farm: DenoWorkerFarm,
}

impl DenoWrapper {
  pub fn new(worker_farm: DenoWorkerFarm) -> Self {
    Self { worker_farm }
  }

  pub fn ping(
    &mut self,
    payload: usize,
  ) {
    let (tx, rx) = channel::<DenoPingResponse>();
    self.worker_farm.send(DenoMachRequest::Ping(payload, tx));
    rx.recv().unwrap();
  }

  pub fn ping_all(&mut self) {
    let (tx, rx) = channel::<DenoPingResponse>();
    self.worker_farm.send_all(DenoMachRequest::Ping(42, tx));
    while let Ok(_) = rx.recv() {}
  }

  pub fn resolver_load(
    &mut self,
    specifier: String,
  ) {
    let (tx, rx) = channel::<DenoResolverLoadResponse>();
    self
      .worker_farm
      .send_all(DenoMachRequest::ResolverLoad(specifier, tx));
    while let Ok(_) = rx.recv() {}
  }

  pub fn resolver_run(
    &mut self,
    resolver_id: String,
    dependency_ref: String,
  ) -> () {
    let (tx, rx) = channel::<DenoResolverRunResponse>();
    self.worker_farm.send(DenoMachRequest::ResolverRun(
      resolver_id,
      dependency_ref,
      tx,
    ));
    rx.recv().unwrap();
  }
}
