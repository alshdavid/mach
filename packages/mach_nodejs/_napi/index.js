const napi = require('./index.node')

module.exports.exec = napi.exec
module.exports.MachNapi = napi.MachNapi
module.exports.MachWorkerNapi = napi.MachWorkerNapi
module.exports.ROOT = require('path').resolve(__dirname, '..')
