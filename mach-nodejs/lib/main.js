import napi from './napi.cjs'
import { Resolver } from './resolver.js'

globalThis.Mach = {}
globalThis.Mach.Resolver = Resolver

const EVENT_PING = 0
const EVENT_RESOLVER_REGISTER = 1
const EVENT_RESOLVER_RUN = 2

const resolvers = {}

napi.run(async (_, action) => {
  // console.log(action)
  if (action.id === 0) {
    return 'Ping'
  }
  if (action.id === 1) {
    const specifier = action.data
    try {
      resolvers[specifier] = await import(specifier)
    } catch (error) {
      console.log(error)
    }
    return 'ResolverRegister'
  }
  if (action === EVENT_RESOLVER_RUN) {
    return
  }
  throw new Error("No action")
})
