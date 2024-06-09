/*
  This can be swapped out via configuration
*/
const import_script = (mach_global.import_script = async (src) => {
  if (mach_bundles[src]) {
    return
  }

  let resolve
  const onload = new Promise((res) => {
    resolve = res
  })
  mach_global.addEventListener(
    'bundle_loaded',
    (event) => event.detail === src && resolve(),
  )

  let script = document.querySelector(`script[src="${src}"]`)
  if (!script) {
    script = document.createElement('script')
    script.src = src
    document.head.appendChild(script)
  }

  await onload
  mach_global.removeEventListener(
    'bundle_loaded',
    (event) => event.detail === src,
  )
})
