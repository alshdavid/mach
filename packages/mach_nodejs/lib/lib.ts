import { MachNapi, RpcCallback } from '../_napi/index.js'

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

  constructor(options: MachOptions = {}) {
    this.#internal = new MachNapi({
      rpc: this.#rpc,
      ...options,
    })
  }

  async build(options: MachBuildOptions) {
    return new Promise((res, rej) => this.#internal
      .build(options, (err, data) => err ? rej(err) : res(data)))
  }

  #rpc: RpcCallback = async (err, id, data, done) => {
    console.log([err, id, data, done])
    if (err) {
      return done({ Err: err })
    }
    switch (id) {
      case 0:
        done({ Ok: null })
      default:
        return done({ Err: "No handler specified" })
    }
  }
}

