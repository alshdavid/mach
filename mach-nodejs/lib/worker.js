/*
  This worker communicates with the host process using
  an IPC channel provided from the Rust napi module
*/
import { workerData } from 'node:worker_threads'
import * as types from './types/index.js'
import napi from './napi/index.cjs'
import { Resolver } from './plugins/resolver.js'
import { Dependency } from './plugins/dependency.js'

globalThis.Mach = {}
globalThis.Mach.Resolver = Resolver

const EVENT_PING = 0
const EVENT_RESOLVER_REGISTER = 1
const EVENT_RESOLVER_RUN = 2

/** @type {Record<string, Resolver<unknown>>} */
const resolvers = {}

napi.run(
  workerData.child_sender,
  workerData.child_receiver,
  async (
    /** @type {any} */ _,
    /** @type {types.Action} */ action,
  ) => {

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
    const result = await resolvers[specifier].triggerResolve({ 
      dependency,
      specifier: dependency.specifier,
      get options() { throw new Error('Not implemented') },
      get logger() { throw new Error('Not implemented') },
      // @ts-expect-error
      get pipeline() { throw new Error('Not implemented') },
      get config() { throw new Error('Not implemented') },
    })
    return { 'ResolverRun': { resolve_result: undefined } }
  }

  throw new Error("No action")
})
