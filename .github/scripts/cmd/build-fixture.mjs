import fs from 'node:fs'
import path from 'node:path'
import child_process from 'node:child_process'
import { Paths } from '../platform/paths.mjs'
import { main as build } from './build.mjs'
import { parse } from '../platform/args.mjs'

export function main(args) {
  const FIXTURE = args._[0]
  const FIXTURE_DIR = path.join(Paths.Fixtures, FIXTURE)

  const [CARGO_ARGS] = args._raw.replace(FIXTURE, '').split(' -- ')
  const [,MACH_ARGS] = process.argv.join(' ').split(' -- ')

  const { binary_path } = build(parse(CARGO_ARGS.split(' ')))

  const TESTS = [
    path.join(FIXTURE_DIR, 'src', 'index.html'),
    path.join(FIXTURE_DIR, 'src', 'index.js'),
    path.join(FIXTURE_DIR, 'src', 'index.jsx'),
    path.join(FIXTURE_DIR, 'src', 'index.ts'),
    path.join(FIXTURE_DIR, 'src', 'index.tsx'),
  ]

  for (const test of TESTS) {
    if (!fs.existsSync(test)) {
      continue
    }

    try {
      child_process.execSync(`${binary_path} ${test} ${MACH_ARGS}`, { cwd: FIXTURE_DIR, stdio: 'inherit' })
    } catch (error) {
      
    }
    return
  }

  throw new Error('Cannot find entry')
}