// MODULE COLLECTION START
const modules = {}

modules["d"] = ($$import, $$export)=>{
  const b = 'b';
  $$export["b"] = b;
};

modules["c"] = ($$import, $$export)=>{
  const a = 'a';
  $$export["a"] = a;
};

modules["b"] = ($$import, $$export, $$export_all, $$export_cjs)=>{
  $$export_all("c")
  $$export_all("d")
};

modules["index"] = ($$import, $$export)=>{
    const fromA = $$import("b");
    console.log(fromA);
    console.log($$import("c"))
    console.log($$import("c"))
    $$import("c", "./b.js").then(m => {
      console.log(m)
      console.log($$import("c"))
    })
};
// MODULE COLLECTION END


// Bootstrap function
// Only exists in the entry module and is passed into loaded modules
void function bootstap(initial_modules = {}) {
  const mach_state = (globalThis[Symbol.for("mach_state")] ?? (globalThis[Symbol.for("mach_state")] = { 
    $$: { 'index.js': initial_modules },
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

  mach_state.$ = (id, bundle_id = 'index.js') => {
    // Load bundle
    if (!mach_state.$$[bundle_id]) {
      mach_state.$$[bundle_id] = import(bundle_id).then(({ modules }) => {
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
    const result = run_module(module_fn, mach_state[id])
    delete mach_state.$$[bundle_id][id]
    return result
  }

  // Start app
  mach_state.$("index")
}(modules)
