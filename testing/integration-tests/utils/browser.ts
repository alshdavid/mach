import path from 'node:path';
import child_process from 'node:child_process';
import fs from 'node:fs';
import * as puppeteer from 'puppeteer-core';

export const PATH_BROWSER_SOCKET = path.join(__dirname, '..', '.puppeteer_socket')

export async function connect_to_browser(): Promise<puppeteer.Browser> {
  return puppeteer.connect({
    browserWSEndpoint: fs.readFileSync(PATH_BROWSER_SOCKET, 'utf8')
  })
}

export type BuildOptions = {
  cwd?: string,
  entries?: string | string[]
}

export async function build(options: BuildOptions) {
  return new Promise((resolve, reject) => {
    child_process.exec(`${path.resolve()}`)
  })
}