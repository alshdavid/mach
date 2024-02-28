module.exports.aa = 'aa'
module.exports.bb = 'bb'

console.log('a', module.exports)

setTimeout(() => {
  console.log('a', module.exports)
  module.exports.hello = 'hello'
}, 500)