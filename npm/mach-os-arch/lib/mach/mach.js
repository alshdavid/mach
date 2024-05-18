import * as types from '../../types/index.js'
import napi from '../../platform/native/index.cjs'

export class Mach {
  /** @type {any} */
  #internal
  constructor(/** @type {types.MachOptions} */ _options) {
    this.#internal = new napi.Mach()
  }

  /** @return {Promise<types.BuildReport>} */
  build(/** @type {types.BuildOptions} */ options) {
    napi.startWorker((_, id) => {
      console.log('create worker', id)
    })

    return this.#internal.build(options)
  }

  static build(/** @type {types.MachOptions & types.BuildOptions} */ options) {
    return new Mach(options).build(options)
  }
}
