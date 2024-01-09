// Header 
const $$mach_modules = (globalThis[Symbol.for("mach_modules")] ?? (globalThis[Symbol.for("mach_modules")] = {}))

// Modules
$$mach_modules["d"] = ($$import, $$export)=>{
  const b = 'b';
  $$export["b"] = b;
};

$$mach_modules["c"] = ($$import, $$export)=>{
  const a = 'a';
  $$export["a"] = a;
};

$$mach_modules["b"] = ($$import, $$export, $$export_all, $$export_cjs)=>{
  $$export_all("c")
  $$export_all("d")
};

$$mach_modules["index"] = ($$import, $$export)=>{
    const fromA = $$import("b");
    console.log(fromA);
};

// Import function
// Only exists in the entry module and is passed into loaded modules
$$mach_modules.$ = (id) => {
  // TODO expand this to support dynamically loaded and partial modules
  const import_func = $$mach_modules.$
  
  const module = $$mach_modules[id];
  if (!(module && module.constructor && module.call && module.apply)) {
      return module;
  }

  const exports = {};

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
    const { ['default']: _, ...props } = import_func(src);
    Object.assign(exports, props);
  }

  $$mach_modules[id] = exports;
  module(import_func, exports, export_all, commonjs);
  return exports;
}

// Bootstrap the initial module
$$mach_modules.$("index")
