module.exports.a_1 = 'a_1'

console.log('a:', { ...module.exports })

setTimeout(() => {
  console.log('a:', { ...module.exports })
  module.exports.a_2 = 'a_2'
  console.log('a:', { ...module.exports })
}, 500)
