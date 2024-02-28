(() => {
	if (true) {
		module.exports = require('./a.js');
		module.exports.a = 'a'
		module.exports.b = 'b'
	}

  console.log('index', module.exports)

  setTimeout(() => {
    console.log('index', module.exports)
  }, 700)
})()
