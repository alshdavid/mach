/*
  This can be swapped out via configuration
*/
const import_script = mach_global.import_script = async src => {
  const existing = document.querySelector(`script[src="${src}"]`)
  if (existing && existing.loaded) {
    return
  }
  const script = existing || document.createElement('script')
  const onload = new Promise((res, rej) => {
    script.onload = res
    script.onerror = rej
  })
  if (!existing) {
    script.src = src
    document.head.appendChild(script)
  }
  await onload
}
