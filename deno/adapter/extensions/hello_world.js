import { core } from "ext:core/mod.js";

const {
  op_mach_hello_world,
} = core.ops

globalThis['Mach'] = globalThis['Mach'] || { ops: {} }
globalThis['Mach'].ops.hello_world = op_mach_hello_world
