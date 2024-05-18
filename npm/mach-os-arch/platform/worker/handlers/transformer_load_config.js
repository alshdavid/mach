import * as types from '../types/index.js'

export function transformer_load_config(
  /** @type {Record<string, types.Transformer<unknown>>} */ transformers,
  /** @type {Record<string, unknown>} */ transformers_config
) {
  return async function(
    /** @type {types.TransformerLoadConfigAction} */ { specifier }
  ) {
    const result = await transformers[specifier].triggerLoadConfig?.({
      get config() {
        throw new Error('Not implemented')
      },
      get options() {
        throw new Error('Not implemented')
      },
      get logger() {
        throw new Error('Not implemented')
      },
      get tracer() {
        throw new Error('Not implemented')
      },
    })
    transformers_config[specifier] = result
  }
}
