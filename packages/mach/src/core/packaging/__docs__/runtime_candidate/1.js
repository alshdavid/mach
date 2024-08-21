export const inits = {}

inits[1] = async (module$, require$, load$) => {
  await load$(34) // load dependency bundles
  const value = require$(3) // import * as value from './foo.js'
  console.log(1, value)
}

// Prelude
;(async () => {
  const loaded = {}
  const modules = {}

  const load$ = async (...targets) => {
    const p = []
    
    for (const target of targets)
      p.push(loaded[target] = loaded[target] || import(`/${target}.js`))
    
    
    for (const module of await Promise.all(p))
      Object.assign(inits, module.inits)
  }

  const require$ = (specifier) => {
    if (!modules[specifier]) {
      inits[specifier](modules[specifier] = {}, require$, load$)
      delete inits[specifier]
    }
    return modules[specifier]
  }

  require$(2)

  window._debug = {
    require$, 
    load$,
    loaded,
    modules,
    inits,
  }
})()
