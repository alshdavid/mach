import * as types from '../types/index.js'

export async function resolver_register(
  /** @type {Record<string, types.Resolver<unknown>>} */ resolvers,
  /** @type {types.ResolverRegisterAction} */ { specifier }
) {
  resolvers[specifier] = (await import(specifier)).default
}