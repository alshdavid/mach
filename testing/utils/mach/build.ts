import path from 'node:path';
import child_process from 'node:child_process';
import fs from 'node:fs';
import * as url from 'node:url'
import fsAsync from 'node:fs/promises';

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))
export const FIXTURES = (...segments: string[]) => path.resolve(__dirname, '..', '..', 'fixtures', ...segments)

export type BuildOptions = {
  cwd: string,
  entries?: string[],
}

export type BuildReport = {
  assets: Record<string, Buffer>
}

export async function build_mach(options: BuildOptions): Promise<BuildReport> {
  const mach_bin = path.join(options.cwd, 'node_modules', '.bin', 'mach')
  const entries = options.entries?.join(' ') || ''

  if (!fs.existsSync(options.cwd)) {
    throw new Error(`Error Does Not Exist: ${options.cwd}`)
  }

  if (!fs.existsSync(path.join(options.cwd, 'node_modules'))) {
    await new Promise((resolve, reject) => {
      child_process.exec(`npm install --no-package-lock`, { cwd: options.cwd }, (err, stdout) => {
        if (err) return reject(err)
        resolve(stdout)
      })
    })
  }

  await new Promise((resolve, reject) => {
    child_process.exec(`${mach_bin} build -c ${entries}`, { cwd: options.cwd }, (err, stdout) => {
      if (err) return reject(err)
      resolve(stdout)
    })
  })
  const assets: Record<string, Buffer> = {}

  for (const filename of await fsAsync.readdir(path.join(options.cwd, 'dist'))) {
    Object.defineProperty(assets, filename, { 
      enumerable: true,
      get: () => fs.readFileSync(path.join(options.cwd, 'dist', filename)) 
    })
  }

  return {
    assets
  }
}