import { Resolver } from '@alshdavid/mach'

export default new Resolver({
  resolve({ dependency }) {
    console.log(dependency)
    return null
  }
})
