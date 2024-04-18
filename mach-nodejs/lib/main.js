import napi from './napi.cjs'

// setTimeout(() => napi.onPing(() => {}))

setTimeout(() => napi.onResolverRegister((specifier) => {
  console.log(1)
  console.log(specifier)
}))

setTimeout(() => napi.onResolverRegister((specifier) => {
  console.log(2)
  console.log(specifier)
}))
