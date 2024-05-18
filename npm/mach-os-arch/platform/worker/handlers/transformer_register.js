import * as types from '../types/index.js'
import { Transformer } from '../plugins/index.js'

export async function transformer_register(
  /** @type {Record<string, Transformer<unknown>>} */ transformers,
  /** @type {types.TransformerRegisterAction} */ { specifier }
) {
  transformers[specifier] = (await import(specifier)).default
}
