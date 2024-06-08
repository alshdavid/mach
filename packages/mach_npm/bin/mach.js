#!/usr/bin/env node

// Infer the binary based on the OS and Arch
const OS = {
  win32: 'windows',
  darwin: 'macos',
  linux: 'linux',
}[process.platform]

const ARCH = {
  arm64: 'arm64',
  x64: 'amd64',
}[process.arch]

// If no binary is selected, gracefully exit
if (!OS && !ARCH) {
  console.warn(
    'Could not find Mach binary for your system. Please compile from source',
  )
  console.warn(
    'Override the built in binary by setting the $MACH_BIN_OVERRIDE environment variable',
  )
  process.exit(0)
}

try {
  await import(`@alshdavid/mach-${OS}-${ARCH}/bin/index.js`)
} catch (error) {
  const fs = await import('node:fs/promises')
  const path = await import('node:path')
  const url = await import('node:url')

  const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

  /** @type {any} */
  const package_json = JSON.parse(
    await fs.readFile(path.join(__dirname, '..', 'package.json'), 'utf8'),
  )
  if (package_json.version !== '0.0.0-local') {
    throw error
  }

  await import('@alshdavid/mach-os-arch/bin/index.js')
}
