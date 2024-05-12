/*
  This worker communicates with the host process using
  an IPC channel provided from the Rust napi module
*/
import { workerData } from 'node:worker_threads'
import napi from './napi/index.cjs'
import { Resolver } from './plugins/resolver.js'

globalThis.Mach = {}
globalThis.Mach.Resolver = Resolver

const EVENT_PING = 0
const EVENT_RESOLVER_REGISTER = 1
const EVENT_RESOLVER_RUN = 2

const resolvers = {}

napi.run(
  workerData.child_sender,
  workerData.child_receiver,
  async (_, action) => {
  // console.log('[JS] Action:', Object.keys(action)[0])

  if ('Ping' in action) {
    return 'Ping'
  }

  if ('ResolverRegister' in action) {
    const { specifier } = action.ResolverRegister
    resolvers[specifier] = await import(specifier)
    return { 'ResolverRegister': {} }
  }

  if ('ResolverRun' in action) {
    return { 'ResolverRun': { resolve_result: { file_path: "/test" }} }
  }

  throw new Error("No action")
})
