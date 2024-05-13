import { Resolver } from '@alshdavid/mach'

export default new Resolver({
  loadConfig() {
    return true
  },
  resolve({ dependency, config }) {
    if (typeof dependency.id === 'undefined') {
      throw new Error('No dependency')
    }
    if (typeof config !== 'boolean' || config !== true) {
      throw new Error('failed to set config')
    }
    return null
  }
})
