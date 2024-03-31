use deno_core::*;

#[derive(Default)]
pub struct LoadResolverState {
  pub load_resolver_callback: Option<v8::Global<v8::Function>>,
}

#[op2]
pub fn op_mach_load_resolver(
  #[state] state: &mut LoadResolverState,
  #[global] f: v8::Global<v8::Function>,
) {
  state.load_resolver_callback.replace(f);
}

deno_core::extension!(
  mach_load_resolver,
  ops = [op_mach_load_resolver],
  esm_entry_point = "ext:mach_load_resolver/extensions/load_resolver.js",
  esm = ["extensions/load_resolver.js"],
  state = |state| {
    state.put(LoadResolverState::default());
  },
);
