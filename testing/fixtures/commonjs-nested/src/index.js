(() => {
	if (true) {
		module.exports.i_0 = 'i_0'
    console.log('i:', {...module.exports})

		module.exports = require('./a.js');
		module.exports.i_1 = 'i_1'
	}

  console.log('i:', {...module.exports})

  setTimeout(() => {
    console.log('i:', {...module.exports})
  }, 700)
})()
