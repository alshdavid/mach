export let Mach

try {
  _apply_exports(await import('./lib.js'))
} catch (error) {
  const fs = await import('node:fs/promises')
  const path = await import('node:path')
  const url = await import('node:url')

  const __dirname = url.fileURLToPath(new URL('.', import.meta.url))
  const package_json_path = path.join(__dirname, '..', 'package.json')
  const package_json = JSON.parse(await fs.readFile(package_json_path, 'utf8'))

  if (package_json.version !== '0.0.0-local') {
    throw error
  }

  ;(await import('tsx/esm/api')).register()
  _apply_exports(await import('./lib.ts'))
}

export function _apply_exports(mach) {
  Mach = mach.Mach
}
