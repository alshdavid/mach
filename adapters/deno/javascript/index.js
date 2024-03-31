import { Resolver } from './resolver.js'
import { Dependency } from './dependency.js'

globalThis.Mach.Resolver = Resolver

const resolvers = new Map()

globalThis.Mach.ops.load_resolver(async specifier => {
  const { default: resolver } = await import(specifier)
  resolvers.set(specifier, resolver)
})

globalThis.Mach.ops.run_resolver_resolve(async ([specifier, dependency_ref]) => {
  const resolver = resolvers.get(specifier)
  const dependency = new Dependency(dependency_ref)
  await resolver.resolve({dependency})
  globalThis.Mach.ops.getter_dependency(dependency_ref, '_close')
})
