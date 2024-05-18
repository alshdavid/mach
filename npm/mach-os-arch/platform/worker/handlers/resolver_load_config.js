import * as types from '../types/index.js'

export function resolver_load_config(
  /** @type {Record<string, types.Resolver<unknown>>} */ resolvers,
  /** @type {Record<string, unknown>} */ resolver_config,
) {
  return async function(
    /** @type {types.ResolverLoadConfigAction} */ { specifier }
  ) {
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
  }
}