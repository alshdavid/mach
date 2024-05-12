import { Resolver } from '@alshdavid/mach'
import enhancedResolve from 'enhanced-resolve'
import * as path from 'node:path'

export default new Resolver({
  resolve({ dependency, specifier }) {
    return new Promise(resolve => {
      enhancedResolve(path.dirname(dependency.resolveFrom), specifier, (err, result) => {
        if (err) {
          console.log('miss')
          resolve(undefined)
          return
        }
        resolve({
          filePath: result
        })
      })
    })
  }
})
