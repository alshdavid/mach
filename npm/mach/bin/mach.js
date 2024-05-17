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
  await import(`@alshdavid/mach-${OS}-${ARCH}/bin/mach.js`)
} catch (error) {
  try {
    await import(`@alshdavid/mach-os-arch/bin/mach.js`)
  } catch (err) {
    throw error
  }
}
