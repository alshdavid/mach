use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

use deno_core::*;
use once_cell::sync::Lazy;
use serde::Serialize;

use crate::message_type;

use mach_libdeno::DenoMachRequest;
use mach_libdeno::DenoPingResponse;
use mach_libdeno::DenoResolverLoadResponse;
use mach_libdeno::DenoResolverRunResponse;

pub static ON_CONNECT: Lazy<
  Mutex<(
    Sender<Sender<DenoMachRequest>>,
    Option<Receiver<Sender<DenoMachRequest>>>,
  )>,
> = Lazy::new(|| {
  let (tx, rx) = channel();
  Mutex::new((tx, Some(rx)))
});

#[op2(reentrant)]
pub fn op_mach_connect<'s>(
  scope: &'s mut v8::HandleScope,
  callback: v8::Local<v8::Function>,
) {
  let (tx, rx) = channel::<DenoMachRequest>();

  ON_CONNECT.lock().unwrap().0.send(tx).unwrap();

  while let Ok(msg) = rx.recv() {
    match msg {
      DenoMachRequest::Ping(number, resolve) => {
        let recv = v8::undefined(scope);
        let value = serde_v8::to_v8(scope, &[message_type::PING, number]).unwrap();
        callback.call(scope, recv.into(), &[value]);
        resolve.send(DenoPingResponse {}).unwrap();
      }
      DenoMachRequest::ResolverLoad(specifier, resolve) => {
        let recv = v8::undefined(scope);
        let value = serde_v8::to_v8(scope, &(message_type::RESOLVER_LOAD, specifier)).unwrap();
        callback.call(scope, recv.into(), &[value]);
        resolve.send(DenoResolverLoadResponse {}).unwrap();
      }
      DenoMachRequest::ResolverRun(resolver_id, dependency_id, resolve) => {
        let recv = v8::undefined(scope);
        let value = serde_v8::to_v8(
          scope,
          &(message_type::RESOLVER_RUN, resolver_id, dependency_id),
        )
        .unwrap();
        callback.call(scope, recv.into(), &[value]);
        resolve.send(DenoResolverRunResponse {}).unwrap();
      }
    }
  }
}

deno_core::extension!(
  mach_connect,
  ops = [op_mach_connect],
  esm_entry_point = "ext:mach_connect/src/mach_extensions/connect/connect.js",
  esm = ["src/mach_extensions/connect/connect.js"],
);
