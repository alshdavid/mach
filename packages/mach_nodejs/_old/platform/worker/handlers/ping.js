import { WorkerState } from '../state.js'
import * as types from '../types/index.js'

export async function ping(
  /** @type {WorkerState} */ _state,
  /** @type {types.PingAction} */ _payload,
) {
  // Do nothing
}
