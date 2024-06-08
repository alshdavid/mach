import { WorkerState } from '../state.js'
import * as types from '../types/index.js'

export async function resolver_load_config(
  /** @type {WorkerState} */ { resolvers, resolver_config },
  /** @type {types.ResolverLoadConfigAction} */ { specifier },
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
