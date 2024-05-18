import * as types from '../types/index.js'
import { WorkerState } from '../state.js'

export async function transformer_register(
  /** @type {WorkerState} */ { transformers },
  /** @type {types.TransformerRegisterAction} */ { specifier },
) {
  transformers[specifier] = (await import(specifier)).default
}
