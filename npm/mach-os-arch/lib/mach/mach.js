import * as types from '../../types/index.js'
import napi from '../../platform/native/index.cjs'

/** @type {typeof types.Mach} */
const MachNapi = napi.Mach;

export class Mach extends MachNapi {
  static build(/** @type {types.MachOptions & types.BuildOptions} */ options) {
    return new Mach(options).build(options)
  }
}
