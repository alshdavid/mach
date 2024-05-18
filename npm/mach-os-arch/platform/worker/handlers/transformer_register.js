import * as types from '../types/index.js'
import { Transformer } from '../plugins/index.js'

export function transformer_register(
  /** @type {Record<string, Transformer<unknown>>} */ transformers,
) {
  return async function(
    /** @type {types.TransformerRegisterAction} */ { specifier }
  ) {
    transformers[specifier] = (await import(specifier)).default
  }
}
