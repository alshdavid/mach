console.log(globalThis.Mach)

console.log(globalThis.Mach.ops)

globalThis.Mach.ops.hello_world(value => {
  console.log('from callback', value)
})
