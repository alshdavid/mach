import * as mach from '@alshdavid/mach'

/**
 * @class
 * @template T
 * @implements {mach.IResolver<T>}
 */
export class Resolver {
  triggerResolve
  triggerLoadConfig
  
  constructor(
    /** @type {mach.ResolverInitOpts<T>} */ options
  ) {
    this.triggerResolve = options.resolve
    this.triggerLoadConfig = options.loadConfig
  }
}
