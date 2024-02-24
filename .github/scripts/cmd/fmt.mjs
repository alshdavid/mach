import child_process from 'node:child_process'
import { Paths } from '../platform/paths.mjs'

export function main(args) {
  child_process.execSync(`pnpm install`, { cwd: Paths.Root, stdio: 'inherit' })
  child_process.execSync(`cargo +nightly fmt`, { cwd: Paths.Root, stdio: 'inherit' })
}