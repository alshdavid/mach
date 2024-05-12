import * as types from '../types/index.js'

/**
 * @class
 * @template T
 * @implements {types.IResolver<T>}
 */
export class Resolver {
  triggerResolve
  triggerLoadConfig
  
  constructor(
    /** @type {types.ResolverInitOpts<T>} */ options
  ) {
    this.triggerResolve = options.resolve
    this.triggerLoadConfig = options.loadConfig
  }
}
