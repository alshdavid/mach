let ready
globalThis.onready = new Promise(res => { ready = res })

const { a1, a2, a3, a4_ident, a4_ident_1, a4_ident_2, a5 } = require('./a')
const { b1, b2, b3, b4_ident, b4_ident_1, b5 } = require('./b')
const { c1 } = require('./c')

globalThis.a1 = a1
globalThis.a2 = a2
globalThis.a3 = a3
globalThis.a4_ident = a4_ident
globalThis.a4_ident_1 = a4_ident_1
globalThis.a4_ident_2 = a4_ident_2
globalThis.a5 = typeof a5

globalThis.b1 = b1
globalThis.b2 = b2
globalThis.b3 = b3
globalThis.b4_ident = b4_ident
globalThis.b4_ident_1 = b4_ident_1
globalThis.b5 = typeof b5

globalThis.c1 = c1

// Nested CJS that sets global properties
setTimeout(() => {
  const { b1, b2, b3, b4_ident, b4_ident_1, b5, b6 } = require('./b-nested')

  globalThis.nested_b1 = b1
  globalThis.nested_b2 = b2
  globalThis.nested_b3 = b3
  globalThis.nested_b4_ident = b4_ident
  globalThis.nested_b4_ident_1 = b4_ident_1
  globalThis.nested_b5 = typeof b5
  globalThis.nested_b6 = typeof b6

  ready()
}, 0);
