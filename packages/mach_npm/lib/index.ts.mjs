// This is for development and will be removed when Nodejs stabilizes TypeScript support
import { register } from 'tsx/esm/api'
register()

import Mach from './index.js'

export default Mach
export * from './index.js'
