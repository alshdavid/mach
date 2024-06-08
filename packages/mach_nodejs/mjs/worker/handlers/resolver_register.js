import { WorkerState } from '../state.js'
import * as types from '../types/index.js'

export async function resolver_register(
  /** @type {WorkerState} */ { resolvers },
  /** @type {types.ResolverRegisterAction} */ { specifier },
) {
  resolvers[specifier] = (await import(specifier)).default
}
