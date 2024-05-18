import * as types from './types/index.js'
import { Resolver, Transformer } from './plugins/index.js'
import * as handlers from './handlers/index.js'

/** @type {Record<string, Resolver<unknown>>} */
const resolvers = {}

/** @type {Record<string, unknown>} */
const resolver_config = {}

/** @type {Record<string, Transformer<unknown>>} */
const transformers = {}

/** @type {Record<string, unknown>} */
const transformers_config = {}

export async function worker_callback(/** @type {types.Action} */ payload) {
  const [action, data] = payload
  let result = undefined

  if (action === 0) result = await handlers.ping(data.Ping)
  else if (action === 1)
    result = await handlers.resolver_register(resolvers, data.ResolverRegister)
  else if (action === 2)
    result = await handlers.resolver_load_config(
      resolvers,
      resolver_config,
      data.ResolverLoadConfig,
    )
  else if (action === 3)
    result = await handlers.resolver_resolve(
      resolvers,
      resolver_config,
      data.ResolverResolve,
    )
  else if (action === 4)
    result = await handlers.transformer_register(
      transformers,
      data.TransformerRegister,
    )
  else if (action === 5)
    result = await handlers.transformer_load_config(
      transformers,
      transformers_config,
      data.TransformerLoadConfig,
    )
  else if (action === 6)
    result = await handlers.transformer_transform(
      transformers,
      transformers_config,
      data.TransformerTransform,
    )

  return [action, result || {}]
}
