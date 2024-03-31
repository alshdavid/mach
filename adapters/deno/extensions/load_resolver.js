import { core } from "ext:core/mod.js";

const {
  op_mach_load_resolver,
} = core.ops

globalThis['Mach'] = globalThis['Mach'] || { ops: {} }
globalThis['Mach'].ops.load_resolver = op_mach_load_resolver
