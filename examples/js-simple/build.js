const napi = require('./node_modules/@alshdavid/mach/cmd/nodejs/napi/index.node')
console.log(napi)

// napi.runCli(["build"])

const mach = new napi.Mach()


mach.build({
  
})
console.log(mach)