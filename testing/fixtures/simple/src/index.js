export { a } from './a.js'
import c, { a } from './a.js'
import { b } from './a.js'
import * as d from './a.js'

console.log(a)
console.log(b)
console.log(c)
console.log(d)
