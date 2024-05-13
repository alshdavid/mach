/*
  This worker communicates with the host process using
  an IPC channel provided from the Rust napi module
*/
import { workerData } from 'node:worker_threads'
import * as types from './types/index.js'
import napi from './napi/index.cjs'
import { Resolver } from './plugins/resolver.js'
import { Transformer } from './plugins/transformer.js'
import { Dependency } from './plugins/dependency.js'
import { MutableAsset } from './plugins/mutable_asset.js'

globalThis.Mach = {}
globalThis.Mach.Resolver = Resolver
globalThis.Mach.Transformer = Transformer
globalThis.Mach.Dependency = Dependency
globalThis.Mach.MutableAsset = MutableAsset

/** @type {Record<string, Resolver<unknown>>} */
const resolvers = {}

/** @type {Record<string, unknown>} */
const resolver_config = {}

/** @type {Record<string, Transformer<unknown>>} */
const transformers = {}

/** @type {Record<string, unknown>} */
const transformers_config = {}

napi.run(
  workerData.child_sender,
  workerData.child_receiver,
  async (
    /** @type {any} */ err,
    /** @type {types.Action} */ action,
  ) => {
  if (err) {
    console.log('JS ------------ has error')
    console.error(err)
    process.exit(1)
  }

  if ('Ping' in action) {
    return { 'Ping': {} }
  }

  if ('ResolverRegister' in action) {
    const { specifier } = action.ResolverRegister
    resolvers[specifier] = (await import(specifier)).default
    return { 'ResolverRegister': {} }
  }

  if ('ResolverLoadConfig' in action) {
    const { specifier } = action.ResolverLoadConfig
    const result = await resolvers[specifier].triggerLoadConfig?.({
      get config() { throw new Error('Not implemented') },
      get options() { throw new Error('Not implemented') },
      get logger() { throw new Error('Not implemented') },
    })
    resolver_config[specifier] = result
    return { ResolverLoadConfig: {} }
  }

  if ('ResolverResolve' in action) {
    const { specifier, dependency: internalDependency } = action.ResolverResolve
    const dependency = new Dependency(internalDependency)
    const result = await resolvers[specifier].triggerResolve({ 
      dependency,
      specifier: dependency.specifier,
      config: resolver_config[specifier],
      get options() { throw new Error('Not implemented') },
      get logger() { throw new Error('Not implemented') },
      // @ts-expect-error
      get pipeline() { throw new Error('Not implemented') },
    })
    return { ResolverResolve: { resolve_result: result } }
  }

  if ('TransformerRegister' in action) {
    const { specifier } = action.TransformerRegister
    transformers[specifier] = (await import(specifier)).default
    return { TransformerRegister: {} }
  }

  if ('TransformerLoadConfig' in action) {
    const { specifier } = action.TransformerLoadConfig
    const result = await transformers[specifier].triggerLoadConfig?.({
      get config() { throw new Error('Not implemented') },
      get options() { throw new Error('Not implemented') },
      get logger() { throw new Error('Not implemented') },
      get tracer() { throw new Error('Not implemented') },
    })
    transformers_config[specifier] = result
    return { TransformerLoadConfig: {} }
  }

  if ('TransformerTransform' in action) {
    const { specifier, mutable_asset: internalMutableAsset } = action.TransformerTransform
    const mutable_asset = new MutableAsset(internalMutableAsset)
    const result = await transformers[specifier].triggerTransform({ 
      asset: mutable_asset,
      config: transformers_config[specifier], 
      get resolve() { throw new Error('Not implemented') }, 
      get options() { throw new Error('Not implemented') }, 
      get logger() { throw new Error('Not implemented') }, 
      get tracer() { throw new Error('Not implemented') }, 
    })
    return { 'TransformerTransform': { transform_result: result } }
  }

  throw new Error("No action")
})
