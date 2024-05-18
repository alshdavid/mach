import napi from '../../platform/native/index.cjs'

napi.setupWorkers((
  /** @type {number} */ workers
) => {

})


napi.exec(process.argv.slice(2))
