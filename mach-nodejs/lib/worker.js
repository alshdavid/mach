/*
  This worker communicates with the host process using
  an IPC channel provided from the Rust napi module
*/
import { workerData } from 'node:worker_threads'
import napi from './napi/index.cjs'
import { Resolver } from './plugins/resolver.js'
import { Dependency } from './plugins/dependency.js'

let r = new Resolver()

globalThis.Mach = {}
globalThis.Mach.Resolver = Resolver

const EVENT_PING = 0
const EVENT_RESOLVER_REGISTER = 1
const EVENT_RESOLVER_RUN = 2

/** @type {Record<string, Resolver>} */
const resolvers = {}

napi.run(
  workerData.child_sender,
  workerData.child_receiver,
  async (_, action) => {

  if ('Ping' in action) {
    return { 'Ping': {} }
  }

  if ('ResolverRegister' in action) {
    const { specifier } = action.ResolverRegister
    resolvers[specifier] = (await import(specifier)).default
    return { 'ResolverRegister': {} }
  }

  if ('ResolverRun' in action) {
    const { specifier, dependency: internalDependency } = action.ResolverRun
    const dependency = new Dependency(internalDependency)
    const result = await resolvers[specifier].resolve({ dependency })
    return { 'ResolverRun': { resolve_result: undefined } }
  }

  throw new Error("No action")
})
