import { Worker } from 'node:worker_threads'
import * as path from 'node:path'
import * as url from 'node:url'
import * as types from '../../types/index.js'
import napi from '../../platform/native/index.cjs'

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))
export class Mach {
  /** @type {any} */
  #internal
  constructor(/** @type {types.MachOptions} */ _options) {
    this.#internal = new napi.Mach()
  }

  /** @return {Promise<types.BuildReport>} */
  build(/** @type {types.BuildOptions} */ options) {
    napi.startWorker(() => {
      new Worker(path.join(__dirname, '..', '..', 'cmd','napi', 'worker.js'))
    })

    return new Promise((res, rej) => {
      this.#internal.build(
        options,
        (/** @type {any} */ error, /** @type {any} */ success) => {
          if (error) return rej(error)
          res(success)
        },
      )
    })
  }

  static build(/** @type {types.MachOptions & types.BuildOptions} */ options) {
    return new Mach(options).build(options)
  }
}
