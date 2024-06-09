import { MachNapi } from '../_napi/index.js'

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
      rpc: (...args: any[]) => this.#rpc(...args),
      ...options,
    })
  }

  async build(options: MachBuildOptions) {
    return new Promise((res, rej) => this.#internal
      .build(options, (err, data) => err ? rej(err) : res(data)))
  }

  #rpc(...args: any[]) {
    console.log(args)
  }
}
