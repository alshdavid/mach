import { Resolver } from '@alshdavid/mach'

export default new Resolver({
  resolve({ dependency }) {
    console.log("from js resolver", dependency.id)
    return null
  }
})
