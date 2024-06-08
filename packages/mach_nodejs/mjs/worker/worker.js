import * as types from './types/index.js'
import * as handlers from './handlers/index.js'
import { WorkerState } from './state.js'

const state = new WorkerState()

export async function worker_callback(/** @type {types.Action} */ payload) {
  const [n, action] = payload
  let result = undefined

  if (n === 0) {
    const d = action.Ping
    result = await handlers.ping(state, d)
  } else if (n === 1) {
    const d = action.ResolverRegister
    result = await handlers.resolver_register(state, d)
  } else if (n === 2) {
    const d = action.ResolverLoadConfig
    result = await handlers.resolver_load_config(state, d)
  } else if (n === 3) {
    const d = action.ResolverResolve
    result = await handlers.resolver_resolve(state, d)
  } else if (n === 4) {
    const d = action.TransformerRegister
    result = await handlers.transformer_register(state, d)
  } else if (n === 5) {
    const d = action.TransformerLoadConfig
    result = await handlers.transformer_load_config(state, d)
  } else if (n === 6) {
    const d = action.TransformerTransform
    result = await handlers.transformer_transform(state, d)
  }

  return [n, result || {}]
}
