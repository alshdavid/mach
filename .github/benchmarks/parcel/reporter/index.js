const { Reporter } = require('@parcel/plugin')
const fs = require('node:fs')
const path = require('node:path')

let durations = {}
let asset_count = 0

/** @type {Map<string, Set<string>>} */
let viz_dependencies = new Map()
module.exports.default = new Reporter({
  report({ event, options }) {
    if (process.env.HS_REPORT_TRANSFORMS === 'true') {
      if (event.phase === 'bundling') {
        console.log('done')
        process.exit(0);
      }
      return
    }
    if (event.type === 'buildSuccess') {
      let end = Date.now()
      let timings = {
        asset_count,
        transformation: (durations['bundling'] - durations['start']) / 1000,
        bundling: (durations['packaging'] - durations['bundling']) / 1000,
        packaging: (durations['optimizing'] - durations['packaging']) / 1000,
        optimizing: (end - durations['optimizing']) / 1000,
        total: (end - durations['start']) / 1000,
      }
      console.table(timings)
      let lines = []
      for (const [a, bs] of viz_dependencies.entries()) {
        for (const b of bs.values()) {
          lines.push(`  "${a.replace(path.join(options.projectRoot, 'src') + path.sep, '')}" -> "${b.replace(path.join(options.projectRoot, 'src') + path.sep, '')}"`)
        }
      }

      let graph_viz = `digraph {\n${lines.sort((a, b) => a.localeCompare(b, undefined, {sensitivity: 'base'})).join('\n')}\n}`
      fs.mkdirSync(path.join(options.projectRoot, 'reports'), { recursive: true })
      fs.writeFileSync(path.join(options.projectRoot, 'reports', 'graph.dot'), graph_viz, 'utf-8')
      return
    }
    if (event.type === 'buildStart') {
      let timestamp = Date.now()
      durations['start'] = timestamp
      process.stdout.write(`Transforming...\n`);
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
        durations['bundling'] = timestamp
        process.stdout.write(`Bundling...\n`);
        return
      }
      if (event.phase === 'bundled') {
        let current_assets = []
        event.bundleGraph.traverse({
          enter: (ev) => {
            if (ev.type === 'asset') {
              let asset = ev.value

              if (current_assets.length > 0) {
                let current_asset = current_assets[current_assets.length - 1]
                let current_file_path = current_asset
                let new_file_path = asset.filePath
                viz_dependencies.set(current_file_path, (viz_dependencies.get(current_file_path) || new Set()).add(new_file_path))
              }
              
              current_assets.push(asset.filePath)
            }
          },
          exit: (ev) => {
            if (ev.type === 'dependency') return
            current_assets.pop()
          }
        })
        return
      }
      if (event.phase === 'packaging') {
        let timestamp = Date.now()
        durations['packaging'] = timestamp
        process.stdout.write(`Packaging...\n`);
        return
      }
      if (event.phase === 'optimizing') {
        let timestamp = Date.now()
        durations['optimizing'] = timestamp
        process.stdout.write(`Optimizing...\n`);
        return
      }
    }
  }
})
