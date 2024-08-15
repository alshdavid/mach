// This will be removed when Nodejs stabilizes TypeScript support
import { register } from 'tsx/esm/api'
register()

import Mach from './index.mjs'

export default Mach
export * from './index.mjs'