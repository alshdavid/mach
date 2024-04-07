import { Resolver } from './resolver.js'
import { Dependency } from './dependency.js'

const THREAD_ID = new URL(import.meta.url).searchParams.get('i') || 1

globalThis.Mach.plugins = { Resolver }

const resolvers = new Map()

globalThis.Mach.ops.load_resolver(async specifier => {
  const { default: resolver } = await import(specifier)
  resolvers.set(specifier, resolver)
})

globalThis.Mach.ops.run_resolver_resolve(async ([resolver_id, dependency_ref]) => {
  const resolver = resolvers.get(resolver_id)
  const dependency = new Dependency(dependency_ref)
  return await resolver.resolve({dependency})
})
