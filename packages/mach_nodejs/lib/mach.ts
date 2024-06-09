import { Worker } from 'node:worker_threads'
import path from 'node:path'
import { ROOT, MachNapi, RpcCallbackData } from '../_napi/index.js'
import { MachWorker } from './mach_worker.js'

export type MachOptions = {
  threads?: number
  nodeWorkers?: number
}

export type MachBuildOptions = {
  entries: string[]
  projectRoot?: string
  outFolder?: string
  clean?: boolean
  optimize?: boolean
  bundleSplitting?: boolean
}

export class Mach {
  #internal: MachNapi
  #workers: Worker[]

  constructor(options: MachOptions = {}) {
    this.#workers = []
    this.#internal = new MachNapi({
      rpc: (...args: any) => this.#rpc(args),
      ...options,
    })
  }

  async build(options: MachBuildOptions) {
    return new Promise((res, rej) =>
      this.#internal.build(options, (err, data) =>
        err ? rej(err) : res(data),
      ),
    )
  }

  async #rpc([err, id, data, done]: RpcCallbackData) {
    console.log(["M", err, id, data])
    if (err) {
      return done({ Err: err })
    }
    switch (id) {
      case 0:
        done({ Ok: null })
        break
      case 1:
        this.#workers.push(await MachWorker.init())
        done({ Ok: null })
        break
      default:
        // @ts-expect-error
        done({ Err: 'No handler specified' })
    }
  }
}
