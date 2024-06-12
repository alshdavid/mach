const napi = require('./index.node')


module.exports.ROOT = require('path').resolve(__dirname, '..')
module.exports.exec = napi.exec
module.exports.machNapiNew = napi.machNapiNew
module.exports.machNapiBuild = napi.machNapiBuild
module.exports.workerCallback = napi.workerCallback