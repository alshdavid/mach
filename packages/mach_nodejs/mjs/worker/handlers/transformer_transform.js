import { MutableAsset } from '../plugins/mutable_asset.js'
import { WorkerState } from '../state.js'
import * as types from '../types/index.js'

export async function transformer_transform(
  /** @type {WorkerState} */ { transformers, transformers_config },
  /** @type {types.TransformerTransformAction} */ {
    specifier,
    ...internalMutableAsset
  },
) {
  const dependencies = /** @type {Array<any>} */ ([])
  const mutable_asset = new MutableAsset(internalMutableAsset, dependencies)
  const result = await transformers[specifier].triggerTransform({
    asset: mutable_asset,
    config: transformers_config[specifier],
    get resolve() {
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

  if (!result || (Array.isArray(result) && result.length === 0)) {
    return
  }

  return {
    content: internalMutableAsset.content,
    kind: internalMutableAsset.kind,
    dependencies,
  }
}
