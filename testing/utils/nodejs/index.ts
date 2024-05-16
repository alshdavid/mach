import * as worker_threads from 'node:worker_threads'
import * as path from 'node:path'

export type JSONObject =
  | string
  | number
  | boolean
  | null
  | JSONObject[]
  | { [key: string]: JSONObject }


export type NodejsContextOptions = {
  type: 'commonjs' | 'module'
}

export class NodejsContext {
  #worker
  #counter
  #reqs

  constructor(options: NodejsContextOptions) {
    this.#reqs = new Map<number, Promise<any>>()
    this.#counter = 0
    if (options.type === 'commonjs') {
      this.#worker = new worker_threads.Worker(path.join(__dirname, 'worker-commonjs', 'index.js'))
    } else {
      this.#worker = new worker_threads.Worker(path.join(__dirname, 'worker-module', 'index.js'))
    }
  }

  async eval(cb: string | Function, args: JSONObject[] = []) {
    let data = cb
    if (typeof cb === 'function') {
      const fn_args = args.map(arg => JSON.stringify(arg)).join(',')
      data = `(${cb.toString()})(${fn_args})`
    }

    let resolve!: ((value: any) => void)
    let reject!: ((value: any) => void)

    const on_reply = new Promise((res, rej) => { 
      resolve = res 
      reject = rej
    })

    const id = this.#counter += 1
    this.#reqs.set(id, on_reply)

    const fn = (event: any) => {
      const { id: incoming_id, error, data } = event
      if (incoming_id !== id) return
      if (error) return reject(error)
      resolve(data)
    }

    this.#worker.addListener('message', fn)
    this.#worker.postMessage({ id, action: 'eval', data })

    try {
      const reply = await on_reply
      this.#reqs.delete(id)
      this.#worker.removeListener('message', fn)
      return reply
    } catch (error) {
      this.#reqs.delete(id)
      this.#worker.removeListener('message', fn)
      throw error
    }
  }

  import(specifier: string) {
    return this.eval(`import(${specifier})`)
  }

  shutdown() {
    this.#worker.terminate()
  }

  async [Symbol.asyncDispose]() {
    for (const [,req] of this.#reqs.entries()) {
      try {
        await req
      } catch (error) {}
    }
    this.#worker.terminate()
  }

  async [Symbol.dispose]() {
    for (const [,req] of this.#reqs.entries()) {
      try {
        await req
      } catch (error) {}
    }
    this.#worker.terminate()
  }
}
