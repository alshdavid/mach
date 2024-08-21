export const inits = {}

inits[3] = async (module$, require$) => {
  module$.foo = 'hi' // export const foo = 'hi'
  console.log(require$(4).foo)
}

inits[4] = async (module$) => {
  let foo = '4 value' // export let foo = '4 value'
  Object.defineProperty(module$, 'foo', { get: () => foo, set: (v) => foo = v })
}
