use deno_core::error::AnyError;
use deno_core::*;
use libmach::DependencyId;
use libmach::DependencyMapSync;

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
pub fn op_mach_getter_dependency<'s>(
  scope: &'s mut v8::HandleScope,
  state: &mut OpState,
  #[string] dependency_id: &str,
  #[number] key: usize,
) -> Result<v8::Local<'s, v8::Value>, AnyError> {
  let dependency_map = state.borrow_mut::<DependencyMapSync>();
  let dependency_map = dependency_map.read().unwrap();
  let dependency_id = DependencyId::from_string(dependency_id).unwrap();
  let dependency = dependency_map.get(&dependency_id).unwrap();
  
  return match key {
    0 => Ok(serde_v8::to_v8(scope, &dependency.id)?),
    1 => Ok(serde_v8::to_v8(scope, &dependency.specifier)?),
    2 => Ok(serde_v8::to_v8(scope, &dependency.specifier_type)?),
    3 => Ok(serde_v8::to_v8(scope, &dependency.is_entry)?),
    4 => Ok(serde_v8::to_v8(scope, &dependency.priority)?),
    5 => Ok(serde_v8::to_v8(scope, &dependency.source_path)?),
    6 => Ok(serde_v8::to_v8(scope, &dependency.source_asset)?),
    7 => Ok(serde_v8::to_v8(scope, &dependency.resolve_from)?),
    8 => Ok(serde_v8::to_v8(scope, &dependency.imported_symbols)?),
    9 => Ok(serde_v8::to_v8(scope, &dependency.bundle_behavior)?),
    _ => Err(AnyError::msg(format!("Unable to find key: \"{}\"", key))),
  }
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
    dependency_map: DependencyMapSync,
  },
  state = |state, options| {
    state.put(RunResolverResolveState::default());
    state.put::<DependencyMapSync>(options.dependency_map);
  },
);
