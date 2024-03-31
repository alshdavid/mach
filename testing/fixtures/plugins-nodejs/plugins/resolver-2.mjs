import { Resolver } from '@alshdavid/mach'

export default new Resolver({
  resolve({ dependency }) {
    console.log("Resolver 1", dependency.id)
    return null
  }
})
