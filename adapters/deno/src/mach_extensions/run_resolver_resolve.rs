use std::{collections::HashMap, sync::{mpsc::{channel, Sender}, Arc, Mutex}};

use deno_core::{error::AnyError, *};
use tokio::sync::oneshot;

use crate::adapter::DependencyGetters;

#[derive(Default)]
pub struct RunResolverResolveState {
  pub run_resolver_resolve_callback: Option<v8::Global<v8::Function>>,
}

#[op2]
pub fn op_mach_run_resolver_resolve(
  #[state] state: &mut RunResolverResolveState,
  #[global] f: v8::Global<v8::Function>,
) {
  state.run_resolver_resolve_callback.replace(f);
}

#[op2]
#[string]
pub fn op_mach_getter_dependency(
  #[state] state: &mut DependencyGetters,
  #[string] dependency_ref: &str,
  #[string] key: &str,
) -> Result<String, AnyError> {
  let dg = state.lock().unwrap();
  let getter = dg.get(dependency_ref).unwrap();
  let (tx_getter, rx_getter) = channel::<String>();
  getter.send((key.to_string(), tx_getter)).unwrap();
  let result = rx_getter.recv().unwrap();
  return Ok(result);
}

deno_core::extension!(
  mach_run_resolver_resolve,
  ops = [
    op_mach_run_resolver_resolve,
    op_mach_getter_dependency,  
  ],
  esm_entry_point = "ext:mach_run_resolver_resolve/extensions/run_resolver_resolve.js",
  esm = ["extensions/run_resolver_resolve.js"],
  options = {
    dependency_getters: DependencyGetters,
  },
  state = |state, options| {
    state.put(RunResolverResolveState::default());  
    state.put::<DependencyGetters>(options.dependency_getters);
  },
);
