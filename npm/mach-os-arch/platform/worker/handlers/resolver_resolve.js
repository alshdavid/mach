import { Dependency } from '../plugins/dependency.js'
import { WorkerState } from '../state.js'
import * as types from '../types/index.js'

export async function resolver_resolve(
  /** @type {WorkerState} */ { resolvers, resolver_config },
    /** @type {types.ResolverResolveAction} */ { specifier, dependency: internalDependency }
) {
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
    return { resolve_result: result }
}
