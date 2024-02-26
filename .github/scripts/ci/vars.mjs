import * as fs from 'node:fs'
import * as path from 'node:path'
import { execSync } from 'node:child_process'
import { Paths } from '../platform/paths.mjs'

const BRANCH_NAME = (process.env.GITHUB_REF?.split('/') || [undefined, undefined, execSync('git rev-parse --abbrev-ref HEAD', { cwd: Paths.Root, stdio: 'pipe', encoding: 'utf8' })]).slice(2).join('/').trim()

const vars = {
  BRANCH_NAME,
  BIN_HASH: '',
  NPM_HASH: '',
  NPM_TAG: BRANCH_NAME,
  UPDATE_NPM: true,
  UPDATE_BIN: true,
}

if (vars.NPM_TAG === 'main') {
  vars.NPM_TAG = 'latest'
}

vars.BIN_HASH = execSync('git ls-files crates/mach | xargs sha256sum | cut -d" " -f1 | sha256sum | cut -d" " -f1', { cwd: Paths.Root, stdio: 'pipe', encoding: 'utf8' }).trim()
vars.NPM_HASH = execSync('git ls-files npm/mach | xargs sha256sum | cut -d" " -f1 | sha256sum | cut -d" " -f1', { cwd: Paths.Root, stdio: 'pipe', encoding: 'utf8' }).trim()

const response = await fetch(`https://registry.npmjs.org/@alshdavid/mach/${vars.NPM_TAG}`).then(r => r.json())
const tarball = response.dist?.tarball

fs.rmSync(Paths.Output, { recursive: true })
fs.mkdirSync(Paths.Output)
execSync(`wget ${tarball} -O npm.tar.gz`, { cwd: Paths.Output, stdio: 'ignore' })
execSync(`tar -xzf npm.tar.gz`, { cwd: Paths.Output, stdio: 'ignore' })
if (fs.existsSync(path.join(Paths.Output, 'package', 'sources.sha1'))) {
  const hash = fs.readFileSync(path.join(Paths.Output, 'package', 'sources.sha1'), 'utf8')
  if (hash === vars.NPM_HASH) {
    vars.UPDATE_NPM = false
  }
}

let output = ''
for (const [key,value] of Object.entries(vars)) {
  output += `${key}=${value}\n`
}
console.log(output.trim())
