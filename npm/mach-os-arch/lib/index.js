// @ts-nocheck
import napi from '../bin/index.js'

export class Mach extends napi.Mach {
  static build(options) {
    return new Mach(options).build(options)
  }
}
