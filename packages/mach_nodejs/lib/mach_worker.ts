import { RpcWorkerCallbackData, MachWorkerNapi } from '../_napi/index.js'

export class MachWorker {
  #internal: MachWorkerNapi

  constructor() {
    this.#internal = new MachWorkerNapi({
      rpc: (...args: any) => this.#rpc(args),
    })
  }

  async #rpc([err, id, data, done]: RpcWorkerCallbackData) {
    console.log([err, id, data, done])
    if (err) {
      return done({ Err: err })
    }
    switch (id) {
      case 0:
        done({ Ok: null })
        break
      default:
        done({ Err: "No handler specified" })
    }
  }
}

