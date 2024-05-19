import array_anon from './values/array_anon.js'
import * as array_anon_namespace from './values/array_anon.js'
import { array_named } from './values/array_named.js'
import * as array_named_namespace from './values/array_named.js'
// import object_anon from './values/object_anon.js'
// import object_anon from './values/object_named.js'


globalThis.array_anon_typeof = better_typeof(array_anon)
globalThis.array_anon_0 = array_anon[0]
globalThis.array_anon_namespace_default_0 = array_anon_namespace.default[0]

globalThis.array_named_0 = array_named[0]
globalThis.array_named_namespace_array_named_0 = array_named_namespace.array_named[0]

console.log([
  globalThis.array_anon_typeof,
  globalThis.array_anon_0,
  globalThis.array_anon_namespace_default_0,
  globalThis.array_named_0,
  globalThis.array_named_namespace_array_named_0,
])

function better_typeof(target) {
  if (Array.isArray(target)) return 'array'
  if (target && typeof target === "object" && (/^(object|array)$/i.test(target.constructor.name) === false)) return 'class_constructor'
  typeof target
}