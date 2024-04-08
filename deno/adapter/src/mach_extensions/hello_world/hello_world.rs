use deno_core::*;

#[op2(fast)]
pub fn op_mach_hello_world() {
  println!("hello from rust");
}

deno_core::extension!(
  mach_hello_world,
  ops = [op_mach_hello_world],
  esm_entry_point = "ext:mach_hello_world/src/mach_extensions/hello_world/hello_world.js",
  esm = ["src/mach_extensions/hello_world/hello_world.js"],
);
