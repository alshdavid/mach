let ready
globalThis.onready = new Promise(res => { ready = res })

const { a1, a2, a3, a4_ident, a4_ident_1, a4_ident_2, a5 } = require('./a')
const { b1, b2, b3, b4_ident, b4_ident_1, b5 } = require('./b')
const { c1 } = require('./c')

const output = globalThis.output = {}

output.a1 = a1
output.a2 = a2
output.a3 = a3
output.a4_ident = a4_ident
output.a4_ident_1 = a4_ident_1
output.a4_ident_2 = a4_ident_2
output.a5 = typeof a5

output.b1 = b1
output.b2 = b2
output.b3 = b3
output.b4_ident = b4_ident
output.b4_ident_1 = b4_ident_1
output.b5 = typeof b5

output.c1 = c1

// Nested CJS that sets global properties
setTimeout(() => {
  const { b1, b2, b3, b4_ident, b4_ident_1, b5, b6 } = require('./b-nested')

  output.nested_b1 = b1
  output.nested_b2 = b2
  output.nested_b3 = b3
  output.nested_b4_ident = b4_ident
  output.nested_b4_ident_1 = b4_ident_1
  output.nested_b5 = typeof b5
  output.nested_b6 = typeof b6

  ready()
}, 0);
