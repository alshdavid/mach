import napi from './napi.cjs'

const EVENT_PING = 0
const EVENT_RESOLVER_REGISTER = 1
const EVENT_RESOLVER_RUN = 2

const resolvers = {}

napi.run(async (action, payload, response) => {
  console.log(action, payload, response)
  if (action === EVENT_PING) {
    console.log(action, payload, response)
    return response()
  }
  if (action === EVENT_RESOLVER_REGISTER) {
    console.log('hiiii')
    const specifier = payload
    resolvers[specifier] = await import(specifier)
    console.log(resolvers)
    return response()
  }
  if (action === EVENT_RESOLVER_RUN) {
    return response()
  }
  throw new Error("No action")
})

globalThis.Mach = {}
globalThis.Mach.Resolver = class Resolver {
  resolve

  constructor(options) {
    this.resolve = options.resolve
  }
}
