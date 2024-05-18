import { WorkerState } from '../state.js'
import * as types from '../types/index.js'

export async function transformer_load_config(
  /** @type {WorkerState} */ { transformers, transformers_config },
  /** @type {types.TransformerLoadConfigAction} */ { specifier },
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
