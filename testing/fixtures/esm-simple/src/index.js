import array_anon from './values/array_anon.js'
import * as array_anon_namespace from './values/array_anon.js'

import { array_named } from './values/array_named.js'
import * as array_named_namespace from './values/array_named.js'

import class_anon from './values/class_anon.js'
import * as class_anon_namespace from './values/class_anon.js'

import { class_named } from './values/class_named.js'
import * as class_named_namespace from './values/class_named.js'

import object_anon from './values/object_anon.js'
import * as object_anon_namespace from './values/object_anon.js'

const output = globalThis.output = {}

output.array_anon_typeof = better_typeof(array_anon)
output.array_anon_0 = array_anon[0]
output.array_anon_namespace_default_0 = array_anon_namespace.default[0]

output.array_named_typeof = better_typeof(array_named)
output.array_named_0 = array_named[0]
output.array_named_namespace_array_named_0 = array_named_namespace.array_named[0]

output.class_anon_typeof = better_typeof(class_anon)
output.class_anon_new_data = new class_anon().data
output.class_anon_namespace_default_new_data = new class_anon_namespace.default().data

output.class_named_typeof = better_typeof(class_named)
output.class_named_new_data = new class_named().data
output.class_named_namespace_class_named_new_data = new class_named_namespace.class_named().data
output.class_named_namespace_default_new_data = new class_named_namespace.default().data

output.object_anon_typeof = better_typeof(object_anon)
output.object_anon_data = object_anon.data
output.object_anon_namespace_object_anon_data = object_anon_namespace.default.data

console.log(output)

function better_typeof(target) {
  if (Array.isArray(target)) {
    return 'array'
  }
  if (target && typeof target === "object" && (/^(object|array)$/i.test(target.constructor.name) === false)) {
    return 'class_constructor'
  } 
  return typeof target
}