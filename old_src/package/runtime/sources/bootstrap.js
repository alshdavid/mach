void function mach_runtime(initial_modules = {}) {
  const mach_state = (globalThis[Symbol.for("PROJECT_HASH")] ?? (globalThis[Symbol.for("PROJECT_HASH")] = { 
    $$: { 'entry': initial_modules },
  }))

  const run_module = (module, exports) => {
    // Deoptimization for CommonJs
    // Looking at you React...
    function commonjs(expr, key = undefined) {
      exports['default'] = exports['default'] || {};
      if (key !== undefined) {
        exports[key] = expr;
        exports['default'][key] = expr;
      } else {
        exports['default'] = expr;
        if (expr && typeof expr == 'object') {
          for (key in expr) {
            exports[key] = expr[key]
          }
        }
      }
      return exports
    }

    function export_all(src) {
      const { ['default']: _, ...props } = mach_state.$(src);
      Object.assign(exports, props);
    }

    module(mach_state.$, exports, export_all, commonjs);
    return exports;
  }

  mach_state.$ = (id, bundle_id = 'entry') => {
    // Load bundle
    if (!mach_state.$$[bundle_id]) {
      mach_state.$$[bundle_id] = import(`./${bundle_id}.js`).then(({ modules }) => {
        mach_state.$$[bundle_id] = modules
        return mach_state.$(id, bundle_id)
      })
      return mach_state.$$[bundle_id]
    }
    
    // Bundle is loading
    if (mach_state.$$[bundle_id] && typeof mach_state.$$[bundle_id].then === 'function') {
      return mach_state.$$[bundle_id]
    }

    if (typeof mach_state.$$[bundle_id][id] !== 'function') {
      return mach_state[id]
    }

    const module_fn = mach_state.$$[bundle_id][id]
    mach_state[id] = mach_state[id] || {}
    delete mach_state.$$[bundle_id][id]
    const result = run_module(module_fn, mach_state[id])
    return result
  }

  // Start app
  mach_state.$("ENTRY_MODULE")
}(modules)
