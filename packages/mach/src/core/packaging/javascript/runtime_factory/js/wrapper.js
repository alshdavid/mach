;(async () => {
  // Code goes here

  mach_bundles[mach_bundle_src] = true
  mach_global.dispatchEvent(
    new CustomEvent('bundle_loaded', { detail: mach_bundle_src }),
  )
})()
