import { Resolver } from '@alshdavid/mach'

export default new Resolver({
  resolve({ dependency }) {
    console.log("Resolver 2", dependency.id)
    return null
  }
})
