import * as worker_threads from 'node:worker_threads'
import * as path from 'node:path'
import * as url from 'node:url'
import * as fs from 'node:fs'

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

export type JSONObject =
  | string
  | number
  | boolean
  | null
  | JSONObject[]
  | { [key: string]: JSONObject }

export type NodejsContextOptions = {
  type: 'commonjs' | 'module'
  entry?: string
}

export class NodejsContext {
  #ready
  #worker
  #counter
  #reqs

  constructor(options: NodejsContextOptions) {
    this.#reqs = new Map<number, Promise<any>>()
    this.#counter = 0
    if (options.type === 'commonjs') {
      this.#worker = new worker_threads.Worker(
        path.join(__dirname, 'worker-commonjs', 'index.js'),
      )
    } else {
      this.#worker = new worker_threads.Worker(
        path.join(__dirname, 'worker-module', 'index.js'),
      )
    }
    if (options.entry) {
      this.#ready = this.import(options.entry)
    } else {
      this.#ready = Promise.resolve()
    }
  }

  async eval<T extends Array<JSONObject>>(
    cb: string | ((...args: T) => any | Promise<any>),
    args?: T,
  ): Promise<JSONObject | undefined> {
    await this.#ready

    let data = cb
    if (typeof cb === 'function') {
      const fn_args = (args || []).map((arg) => JSON.stringify(arg)).join(',')
      data = `(${cb.toString()})(${fn_args})`
    }

    let resolve!: (value: any) => void
    let reject!: (value: any) => void

    const on_reply = new Promise((res, rej) => {
      resolve = res
      reject = rej
    })

    const id = (this.#counter += 1)
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
      return reply as any
    } catch (error) {
      this.#reqs.delete(id)
      this.#worker.removeListener('message', fn)
      throw error
    }
  }

  async get_global(...keys: string[]): Promise<JSONObject | undefined> {
    return this.eval(
      async (keys = []) => {
        try {
          let curr = globalThis 
          for (const key of keys) {
            if (!(key in curr)) {
              return undefined
            }
            // @ts-expect-error
            curr = await curr[key]
          }
          return await curr
        } catch (error) {
          return undefined
        }
      },
      [keys],
    )
  }

  async resolve_global(key: string): Promise<JSONObject | undefined> {
    return this.eval(
      async (key) => {
        // @ts-expect-error
        return await globalThis[key]
      },
      [key],
    )
  }

  async import(specifier: string) {
    if (specifier.startsWith(path.sep) && !fs.existsSync(specifier)) {
      throw new Error(`Cannot find specifier: ${specifier}`)
    }
    await this.eval(
      async (specifier) => {
        await import(specifier)
      },
      [specifier],
    )
  }

  shutdown() {
    this.#worker.terminate()
  }

  async [Symbol.asyncDispose]() {
    for (const [, req] of this.#reqs.entries()) {
      try {
        await req
      } catch (error) {}
    }
    this.#worker.terminate()
  }

  async [Symbol.dispose]() {
    for (const [, req] of this.#reqs.entries()) {
      try {
        await req
      } catch (error) {}
    }
    this.#worker.terminate()
  }
}
