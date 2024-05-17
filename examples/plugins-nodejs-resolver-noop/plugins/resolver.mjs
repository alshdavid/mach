import { Resolver } from '@alshdavid/mach'

export default new Resolver({
  loadConfig() {
    return 'my plugin config'
  },
  resolve({ dependency, config }) {
    if (typeof dependency.id === 'undefined') {
      throw new Error('No dependency')
    }
    if (typeof config !== 'string' || config !== 'my plugin config') {
      throw new Error('failed to set config')
    }
    return null
  },
})
