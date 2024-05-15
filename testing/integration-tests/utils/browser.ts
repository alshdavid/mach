import path from 'node:path';
import child_process from 'node:child_process';
import fs from 'node:fs';
import fsAsync from 'node:fs/promises';
import * as puppeteer from 'puppeteer-core';

export const PATH_BROWSER_SOCKET = path.resolve(__dirname, '..', '.puppeteer_socket')
export const FIXTURES = (...segments: string[]) => path.resolve(__dirname, '..', '..', 'fixtures', ...segments)

export async function connect_to_browser(): Promise<puppeteer.Browser> {
  return puppeteer.connect({
    browserWSEndpoint: fs.readFileSync(PATH_BROWSER_SOCKET, 'utf8')
  })
}

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