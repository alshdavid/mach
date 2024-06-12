import { Worker } from 'node:worker_threads'
import path from 'node:path'
import { ROOT, machNapiNew, machNapiBuild, RpcCallbackMain, MachNapi } from '../_napi/index.js'
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
  #rpcCallback: any

  constructor(options: MachOptions = {}) {
    this.#workers = []
    this.#rpcCallback = (...args: any) => {this.#rpc(args)}
    this.#internal = machNapiNew({
      rpc: this.#rpcCallback,
      ...options,
    })
  }

  async build(options: MachBuildOptions) {
    return new Promise((res, rej) =>
      machNapiBuild(this.#internal, options, (err, data) =>
        err ? rej(err) : res(data),
      ),
    )
  }

  async #rpc([err, id, data, done]: RpcCallbackMain) {
    console.log(["M", err, id, data, done])
    if (err) {
      return done({ Err: err })
    }
    switch (id) {
      case 0:
        done({ Ok: undefined })
        break
      case 1:
        this.#workers.push(await MachWorker.init())
        done({ Ok: undefined })
        break
      default:
        // @ts-expect-error
        done({ Err: 'No handler specified' })
      }
  }
}
