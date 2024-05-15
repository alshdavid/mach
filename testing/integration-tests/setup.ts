import * as reporter from 'node:test/reporters';
import { run } from 'node:test';
import path from 'node:path';
import fs from 'node:fs';
import * as puppeteer from 'puppeteer-core';
import { PATH_BROWSER_SOCKET } from './utils/browser';

void async function() {
  if (fs.existsSync(PATH_BROWSER_SOCKET)) {
    console.log('Tests are already running')
    return
  }
  const browser = await puppeteer.launch({
    executablePath: '/usr/bin/google-chrome-stable',
    headless: true,
  })

  fs.writeFileSync(PATH_BROWSER_SOCKET, browser.wsEndpoint(), 'utf8')

  run({ 
    files: [path.resolve('./tests/javascript.ts')],
  })
    .on('test:fail', () => {
      process.exitCode = 1;
    })
    .on('end', async () => {
      await browser.close()
      fs.rmSync(PATH_BROWSER_SOCKET)
    })
    .compose(new reporter.spec())
    .pipe(process.stdout)
}()

 