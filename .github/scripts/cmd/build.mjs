import fs from 'node:fs'
import path from 'node:path'
import child_process from 'node:child_process'
import { Paths } from '../platform/paths.mjs'

const target_map = {
  'x86_64-unknown-linux-gnu': 'linux-amd64',
  'aarch64-unknown-linux-gnu': 'linux-arm64',
  'x86_64-apple-darwin': 'macos-amd64',
  'aarch64-apple-darwin': 'macos-arm64',
  'x86_64-pc-windows-msvc': 'windows-amd64',
  'aarch64-pc-windows-msvc': 'windows-arm64',
}

export function main(args) {
  if (!fs.existsSync(Paths.Root, 'node_modules')) {
    child_process.execSync(`pnpm install`, { cwd: Paths.Root, stdio: 'inherit' })
  }

  const BIN_VERSION = process.env.BIN_VERSION || ''
  const PROFILE = args.profile ? args.profile : args.release ? 'release' : 'debug'
  const TARGET = args.target
  const TARGET_DIR = target_map[TARGET] || TARGET

  if (TARGET) {
    console.table({ PROFILE, TARGET, BIN_VERSION, ARGS: args._raw })
  }

  const __cargo_output = path.join(Paths.CargoOutput, ...[TARGET, PROFILE].filter(x => x))
  const __cargo_output_binary = path.join(__cargo_output, process.platform !== 'win32' ? 'mach' : 'mach.exe')
  const __output = path.join(Paths.Output, ...[TARGET_DIR, PROFILE].filter(x => x))

  fs.rmSync(__output, { force: true, recursive: true })

  if (BIN_VERSION) {
    console.log("Updating bin version", BIN_VERSION)
    const [branch, buildno] = BIN_VERSION.split('.')
    const verno = branch === 'main' ? `0.0.${buildno}` : `0.0.${buildno}-${branch}`
    console.log(verno)
    const toml = fs.readFileSync(path.join(Paths.Root, 'mach', 'Cargo.toml'), 'utf8')
    const updated = toml.replace('version = "0.0.0-local"', `version = "${verno}"`)
    fs.writeFileSync(path.join(Paths.Root, 'mach', 'Cargo.toml'), updated, 'utf8')
  }

  console.log(`cargo build ${args._raw || ''}`)
  child_process.execSync(`cargo build ${args._raw || ''}`, { cwd: Paths.Root, stdio: 'inherit' })

  if (TARGET) {
    console.log(Paths.CargoOutput)
    for (const dir of fs.readdirSync(Paths.CargoOutput)) console.log(dir)
    console.log()
    console.log(path.join(Paths.CargoOutput, TARGET))
    for (const dir of fs.readdirSync(path.join(Paths.CargoOutput, TARGET))) console.log(dir)
  }

  fs.mkdirSync(path.join(__output), { recursive: true })
  fs.mkdirSync(path.join(__output, 'lib'), { recursive: true })
  fs.mkdirSync(path.join(__output, 'bin'), { recursive: true })

  fs.cpSync(path.join(Paths.Root, 'npm', 'node-adapter'), path.join(__output, 'lib', 'node_adapter'), { recursive: true })
  fs.rmSync(path.join(__output, 'lib', 'node_adapter', 'node_modules'), { force: true, recursive: true })

  let binary_path = path.join(__output, 'bin', 'mach')
  if (process.platform === 'win32') {
    binary_path += '.exe'
  }

  fs.cpSync(__cargo_output_binary, binary_path)
  if (TARGET === undefined && PROFILE === 'debug') {
    let new_binary_path = path.join(__output, 'bin', 'machd')
    if (process.platform === 'win32') {
      new_binary_path += '.exe'
    }
    fs.cpSync(binary_path, new_binary_path)
    fs.rmSync(binary_path)
    binary_path = new_binary_path
  }
  if (TARGET === undefined && PROFILE === 'release') {
    let new_binary_path = path.join(__output, 'bin', 'machr')
    if (process.platform === 'win32') {
      new_binary_path += '.exe'
    }
    fs.cpSync(binary_path, new_binary_path)
    fs.rmSync(binary_path)
    binary_path = new_binary_path
  }

  return {
    output_dir: __output,
    binary_path,
  }
}
