import { Transformer } from '../public/transformer'
import { Resolver } from '../public/resolver'

export type RequestLoadPlugin = {
  plugin_key: string,
  specifier: string,
}

export async function load_plugin(
  transformers: Map<string, Transformer>,
  resolvers: Map<string, Resolver>,
  { specifier }: RequestLoadPlugin,
) {
  const module = await import(specifier)
  if (module.default instanceof Transformer) {
    transformers.set(specifier, module.default)
  }
  else if (module.default instanceof Resolver) {
    resolvers.set(specifier, module.default)
  } else {
    throw new Error('Unable to load plugin')
  }
}
