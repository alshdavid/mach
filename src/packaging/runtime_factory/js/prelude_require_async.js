const require_async = async (bundle_ids, module_id) => {
  if (mach_modules[module_id]) {
    return mach_modules[module_id]
  }
  
  const loading =  []
  for (const bundle_id of bundle_ids) {
    loading.push(import_script(mach_manifest[bundle_id]))
  }
  await Promise.all(loading)

  const module = { exports: {} }
  mach_modules[module_id] = module.exports

  const define_export = (key, getter, setter) => {
    Object.defineProperty(module.exports, key, { get: getter, set: setter, enumerable: true, configurable: true });
  }

  await mach_init[module_id](define_export, require_async, module, module.exports)
  
  mach_init[module_id] = undefined
  return mach_modules[module_id]
}
