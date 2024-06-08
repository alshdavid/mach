import { Resolver, Transformer } from './plugins/index.js'

export class WorkerState {
  /** @type {Record<string, Resolver<unknown>>} */
  resolvers = {}

  /** @type {Record<string, unknown>} */
  resolver_config = {}

  /** @type {Record<string, Transformer<unknown>>} */
  transformers = {}

  /** @type {Record<string, unknown>} */
  transformers_config = {}
}
