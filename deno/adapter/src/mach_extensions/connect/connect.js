import { core } from "ext:core/mod.js";

const {
  op_mach_connect,
} = core.ops

globalThis['Mach'] = globalThis['Mach'] || { ops: {} }
globalThis['Mach'].ops.connect = op_mach_connect
