import * as types from '../types/index.js'

/**
 * @class
 * @template T
 * @implements {types.ITransformer<T>}
 */
export class Transformer {
  triggerLoadConfig
  triggerTransform

  constructor(
    /** @type {types.TransformerInitOpts<T>} */ options
  ) {
    this.triggerLoadConfig = options.loadConfig
    this.triggerTransform = options.transform
    if (options.canReuseAST) throw new Error('Feature not supported')
    if (options.parse) throw new Error('Feature not supported')
    if (options.postProcess) throw new Error('Feature not supported')
    if (options.generate) throw new Error('Feature not supported')
  }
}
