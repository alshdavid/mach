process.stdout.write({
  os: {
    "windows": "win32",
    "linux": "linux",
    "macos": "darwin",
  },
  arch: {
    "arm64": "arm64",
    "amd64": "x64",
  },
  bin: {
    "windows": ".\\bin\\mach.exe",
    "linux": "./bin/mach",
    "macos": "./bin/mach",
  }
}[process.argv[2]][process.argv[3]])
