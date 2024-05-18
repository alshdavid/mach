/*
  This worker communicates with the host process using
  an IPC channel provided from the Rust napi module
*/
import { workerData } from 'node:worker_threads'
import * as types from './types/index.js'
import napi from '../napi/index.cjs'
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

napi.worker(
  workerData.child_sender,
  workerData.child_receiver,
  async (/** @type {any} */ err, /** @type {types.Action} */ payload) => {
    try {
      const [action, data] = payload
      if (err) {
        console.log('JS ------------ has error')
        console.error(err)
        process.exit(1)
      }

      if (action === 0) {
        return [action, {}]
      }

      if (action === 1) {
        const { specifier } = data.ResolverRegister
        resolvers[specifier] = (await import(specifier)).default
        return [action, {}]
      }

      if (action === 2) {
        const { specifier } = data.ResolverLoadConfig
        const result = await resolvers[specifier].triggerLoadConfig?.({
          get config() {
            throw new Error('Not implemented')
          },
          get options() {
            throw new Error('Not implemented')
          },
          get logger() {
            throw new Error('Not implemented')
          },
        })
        resolver_config[specifier] = result
        return [action, {}]
      }

      if (action === 3) {
        const { specifier, dependency: internalDependency } =
          data.ResolverResolve
        const dependency = new Dependency(internalDependency)
        const result = await resolvers[specifier].triggerResolve({
          dependency,
          specifier: dependency.specifier,
          config: resolver_config[specifier],
          get options() {
            throw new Error('Not implemented')
          },
          get logger() {
            throw new Error('Not implemented')
          },
          // @ts-expect-error
          get pipeline() {
            throw new Error('Not implemented')
          },
        })
        return [action, { resolve_result: result }]
      }

      if (action === 4) {
        const { specifier } = data.TransformerRegister
        transformers[specifier] = (await import(specifier)).default
        return [action, {}]
      }

      if (action === 5) {
        const { specifier } = data.TransformerLoadConfig
        const result = await transformers[specifier].triggerLoadConfig?.({
          get config() {
            throw new Error('Not implemented')
          },
          get options() {
            throw new Error('Not implemented')
          },
          get logger() {
            throw new Error('Not implemented')
          },
          get tracer() {
            throw new Error('Not implemented')
          },
        })
        transformers_config[specifier] = result
        return [action, {}]
      }

      if (action === 6) {
        const { specifier, ...internalMutableAsset } = data.TransformerTransform

        const dependencies = /** @type {Array<any>} */ ([])
        const mutable_asset = new MutableAsset(
          internalMutableAsset,
          dependencies,
        )
        const result = await transformers[specifier].triggerTransform({
          asset: mutable_asset,
          config: transformers_config[specifier],
          get resolve() {
            throw new Error('Not implemented')
          },
          get options() {
            throw new Error('Not implemented')
          },
          get logger() {
            throw new Error('Not implemented')
          },
          get tracer() {
            throw new Error('Not implemented')
          },
        })

        if (!result || (Array.isArray(result) && result.length === 0)) {
          return [action, {}]
        }

        return [
          action,
          {
            content: internalMutableAsset.content,
            kind: internalMutableAsset.kind,
            dependencies,
          },
        ]
      }

      throw new Error('No action')
    } catch (/** @type {any} */ error) {
      if (error instanceof Error) {
        throw `\n${error.stack}\n`
      }
      if (typeof error === 'string') {
        throw error
      }
      throw 'An error occurred in JavaScript worker'
    }
  },
)
