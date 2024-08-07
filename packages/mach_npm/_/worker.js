// This is required because Nodejs Worker does not inherit `--import tsx`
// https://github.com/privatenumber/tsx/issues/354
//
// Probably won't need this when Nodejs can run TypeScript directly
try {
  await import('../bin/worker.js')
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

  (await import('tsx/esm/api')).register()
  await import('../bin/worker.ts')
}