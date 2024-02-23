/*
  This can be swapped out via configuration
*/
async function import_script(src) {
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
