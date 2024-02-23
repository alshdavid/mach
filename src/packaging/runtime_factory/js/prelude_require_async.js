const require_async = async (bundle_ids, module_id) => {
  if (parcel_modules[module_id]) {
    return parcel_modules[module_id]
  }
  
  const loading =  []
  for (const bundle_id of bundle_ids) {
    loading.push(import_script(parcel_manifest[bundle_id]))
  }
  await Promise.all(loading)

  const module = { exports: {} }
  parcel_modules[module_id] = module.exports

  const define_export = (key, getter, setter) => {
    Object.defineProperty(module.exports, key, { get: getter, set: setter, enumerable: true, configurable: true });
  }

  await parcel_init[module_id](define_export, require_async, module, module.exports)
  
  parcel_init[module_id] = undefined
  return parcel_modules[module_id]
}
