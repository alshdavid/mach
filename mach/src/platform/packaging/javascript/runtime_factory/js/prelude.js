// Most of these are remove at optimization time
const mach_global = globalThis['PROJECT_IDENTIFIER'] = globalThis['PROJECT_IDENTIFIER'] || {}
const mach_init = mach_global.init = mach_global.init || {}
const mach_modules = mach_global.modules = mach_global.modules || {}
const mach_manifest = mach_global.manifest = mach_global.manifest || {}

// Lets optimizer minify these
const document = globalThis.document
const window = globalThis
