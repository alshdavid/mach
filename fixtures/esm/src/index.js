import * as fromA from './a.js'
import * as fromB from './b.js'
import b from './a.js'
import named from './b.js'
import { c } from './b.js'
import { d, d2 } from './d.js'
import { e, e1, e2, e3 } from './e.js'

console.log(fromA)
console.log(fromB.default)
console.log(fromB)
console.log(b)
named()
console.log(c)
console.log({d, d2})
console.log({e, e1, e2, e3})
