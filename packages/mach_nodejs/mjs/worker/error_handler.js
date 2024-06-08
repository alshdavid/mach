import * as types from './types/index.js'

export function error_handler(
  /** @type {(value: types.Action) => Promise<any>} */ callback,
) {
  return async function (
    /** @type {any} */ err,
    /** @type {types.Action} */ payload,
  ) {
    try {
      if (err) {
        console.log('JS ------------ has error')
        console.error(err)
        process.exit(1)
      }
      return await callback(payload)
    } catch (/** @type {any} */ error) {
      if (error instanceof Error) {
        throw `\n${error.stack}\n`
      }
      if (typeof error === 'string') {
        throw error
      }
      throw 'An error occurred in JavaScript worker'
    }
  }
}
