import { Resolver, Dependency } from '../public/resolver'

export type RequestRunResolver = {
  plugin_key: string,
  dependency: Dependency
}

export async function run_resolver(resolver: Resolver, { plugin_key, dependency }: RequestRunResolver) {
  const result = await resolver.init.resolve({ dependency })
  if (result === null || result === undefined) {
    return {}
  }
  return result
}