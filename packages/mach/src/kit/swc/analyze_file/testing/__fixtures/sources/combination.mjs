import './foo'
import import_foo_default from './foo'
import { import_foo_1, import_bar_1 } from './foo'
import { import_foo_2 as import_foo_2_renamed } from './foo'
import * as import_foo_namespace from './foo'

const foo_1 = ['', 42]
const foo_2 = ['']
const foo_3 = { 1: '' }
const foo_4 = { foo_4_1: '' }
const foo_5 = { foo_5_1: '' }
const foo_6 = { foo_6_1: '' }
const foo_7 = ''
const foo_8 = ''

export default class {}
export const [foo_1_1, foo_1_2] = foo_1
export const [foo_2_1] = foo_2
export const { [1]: foo_3_1 } = foo_3
export const { ['foo_4_1']: foo_4_1_renamed } = foo_4
export const { foo_5_1: foo_5_1_renamed } = foo_5
export const { foo_6_1 } = foo_6
export class Foo {}
export function foo() {}
export const foo = ''
export { foo_7, foo_8 }
export { foo_7 as foo_7_renamed }

export * from './foo'
export { reexport_foo_1 } from './foo'
export * as reexport_foo_2 from './foo'
export { reexport_foo_3 as reexport_foo_3_renamed } from './foo'
