import * as types from '../types/index.js'

export function resolver_register(
  /** @type {Record<string, types.Resolver<unknown>>} */ resolvers,
) {
  return async function(
    /** @type {types.ResolverRegisterAction} */ { specifier }
  ) {
    resolvers[specifier] = (await import(specifier)).default
  }
}