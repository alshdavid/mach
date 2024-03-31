use deno_core::*;

#[op2(fast)]
pub fn op_mach_load_resolver() {
}

deno_core::extension!(
  mach_load_resolver,
  ops = [op_mach_load_resolver],
  esm_entry_point = "ext:mach_load_resolver/extensions/load_resolver.js",
  esm = ["extensions/load_resolver.js"],
);
