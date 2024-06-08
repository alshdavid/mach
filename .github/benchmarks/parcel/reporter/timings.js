const { Reporter } = require('@parcel/plugin')
const fs = require('node:fs')
const path = require('node:path')

let durations = {
    start: 0,
    bundling: 0,
    packaging: {},
    optimizing: {}
}
let asset_count = 0

module.exports.default = new Reporter({
  report({ event }) {
    if (event.type === 'buildSuccess') {
      let end = Date.now()

      let packagingStartTime = undefined
      let packagingTime = 0

      for (const [publicId, start_time] of Object.entries(durations.packaging)) {
        let end_time = durations.optimizing[publicId]
        if (!end_time) {
            throw new Error('Unable to find end time')
        }
        if (packagingStartTime === undefined || start_time < packagingStartTime) {
            packagingStartTime = start_time
        }
        packagingTime += end_time - start_time
      }

      const totalTime = end - durations.start
      const transformationTime = durations.bundling - durations.start
      const bundlingTime = packagingStartTime - durations.bundling
			const optimizingTime = totalTime - packagingTime - bundlingTime - transformationTime
      
			let timings = {
        asset_count,
        transformation: transformationTime / 1000,
        bundling: bundlingTime / 1000,
        packaging: packagingTime / 1000,
        optimizing: optimizingTime / 1000,
        total: totalTime / 1000,
      }

      console.table(timings)
      return
    }
    if (event.type === 'buildStart') {
      let timestamp = Date.now()
      durations.start = timestamp
      return
    }
    if (event.type === 'buildProgress') {
      if (event.phase === 'resolving') {
        return
      }
      if (event.phase === 'transforming') {
        asset_count += 1
        return
      }
      if (event.phase === 'bundling') {
        let timestamp = Date.now()
        durations.bundling = timestamp
        if (process.env['PARCEL_EXIT_TRANSFORM'] == 'true') {
          const transformationTime = durations.bundling - durations.start
          console.table({
            asset_count,
            transformationTime: transformationTime / 1000
          })
          process.exit()
        }
        return
      }
      if (event.phase === 'packaging') {
        let timestamp = Date.now()
        durations.packaging[event.bundle.publicId] = timestamp
        return
      }
      if (event.phase === 'optimizing') {
        let timestamp = Date.now()
        durations.optimizing[event.bundle.publicId] = timestamp
        return
      }
    }
  }
})
