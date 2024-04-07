import { core } from "ext:core/mod.js";

const {
  op_mach_run_resolver_resolve,
  op_mach_getter_dependency,
} = core.ops

globalThis['Mach'] = globalThis['Mach'] || { ops: {} }
globalThis['Mach'].ops.run_resolver_resolve = op_mach_run_resolver_resolve
globalThis['Mach'].ops.getter_dependency = op_mach_getter_dependency
