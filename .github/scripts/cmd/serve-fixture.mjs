import fs from 'node:fs'
import path from 'node:path'
import child_process from 'node:child_process'
import { Paths } from '../platform/paths.mjs'

export function main(args) {
  child_process.execSync(`pnpm install`, { cwd: Paths.Root, stdio: 'inherit' })

  const FIXTURE = args._.splice(0,1)[0]
  const FIXTURE_PATH = path.join(Paths.Fixtures, FIXTURE)
  
  if (!fs.existsSync(FIXTURE_PATH)) {
    throw new Error('Cannot find fixture')
  }

  child_process.execSync(`npx http-server -c=-1 -p=3000 .`, { cwd: FIXTURE_PATH, stdio: 'inherit' })
}