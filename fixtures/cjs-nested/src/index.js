(() => {
	if (true) {
		module.exports = require('./a.js');
		module.exports.a = ''
		exports.b = ''
		const foo = exports.b
	}
})()
