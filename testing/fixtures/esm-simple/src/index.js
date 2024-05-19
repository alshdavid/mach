import array_anon_default from './values/array_anon.js'
import * as array_anon_namespace from './values/array_anon.js'

import { array_named } from './values/array_named.js'
import * as array_named_namespace from './values/array_named.js'

import class_anon_default from './values/class_anon.js'
import * as class_anon_namespace from './values/class_anon.js'

import { class_named } from './values/class_named.js'
import class_named_default from './values/class_named.js'
import * as class_named_namespace from './values/class_named.js'

import function_anon_arrow_default from './values/function_anon_arrow.js'

import object_anon from './values/object_anon.js'
import * as object_anon_namespace from './values/object_anon.js'

const output = globalThis.output = {}

// Anon array
output.array_anon_default_typeof = better_typeof(array_anon_default)
output.array_anon_default_0 = array_anon_default[0]
output.array_anon_namespace_default_0 = array_anon_namespace.default[0]

// Named array
output.array_named_typeof = better_typeof(array_named)
output.array_named_0 = array_named[0]
output.array_named_namespace_array_named_0 = array_named_namespace.array_named[0]

// Anon class
output.class_anon_default_typeof = better_typeof(class_anon_default)
output.class_anon_default_new_data = new class_anon_default().data
output.class_anon_namespace_default_new_data = new class_anon_namespace.default().data

// Named class
output.class_named_class_named_typeof = better_typeof(class_named)
output.class_named_class_named_new_data = new class_named().data

output.class_named_default_typeof = better_typeof(class_named_default)
output.class_named_default_new_data = new class_named_default().data

output.class_named_namespace_class_named_typeof = better_typeof(class_named_namespace.class_named)
output.class_named_namespace_class_named_new_data = new class_named_namespace.class_named().data

output.class_named_namespace_default_typeof = better_typeof(class_named_namespace.default)
output.class_named_namespace_default_new_data = new class_named_namespace.default().data

// Arrow function anon
output.function_anon_arrow_default_typeof = better_typeof(function_anon_arrow_default)
output.function_anon_arrow_default_return = function_anon_arrow_default()

// Object anon
output.object_anon_typeof = better_typeof(object_anon)
output.object_anon_data = object_anon.data
output.object_anon_namespace_object_anon_data = object_anon_namespace.default.data

console.log(output)

function better_typeof(target) {
  if (Array.isArray(target)) {
    return 'array'
  }
  if (typeof target === 'function' && target.toString().includes('class')) {
    return 'class_constructor'
  }
  if (typeof target === 'function' && target.toString().includes('function'))
  return typeof target
}