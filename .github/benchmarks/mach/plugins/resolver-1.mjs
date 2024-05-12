import { Resolver } from '@alshdavid/mach'

export default new Resolver({
  resolve({ dependency }) {
    if (typeof dependency.id === 'undefined') {
      throw new Error('No dependency')
    }
    return null
  }
})
