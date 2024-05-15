import path from 'node:path';
import fs from 'node:fs';
import * as puppeteer from 'puppeteer-core';

export const PATH_BROWSER_SOCKET = path.resolve(__dirname, '..', '..', '.puppeteer_socket')

let BROWSER: Promise<puppeteer.Browser & AsyncDisposable & Disposable> | undefined

export async function new_browser(): Promise<puppeteer.Browser & AsyncDisposable & Disposable> {
  if (!BROWSER) {
    BROWSER = puppeteer.launch({
      executablePath: '/usr/bin/google-chrome-stable',
      headless: true,
    }).then(browser => {
      fs.writeFileSync(PATH_BROWSER_SOCKET, browser.wsEndpoint(), 'utf8')

      process.on('SIGINT', async() => {
        fs.rmSync(PATH_BROWSER_SOCKET)
        await browser.close() 
      })
    
      // @ts-expect-error
      browser[Symbol.asyncDispose] = async () => {
        fs.rmSync(PATH_BROWSER_SOCKET)
        await browser.close() 
      }
    
      // @ts-expect-error
      browser[Symbol.dispose] = async () => {
        fs.rmSync(PATH_BROWSER_SOCKET)
        await browser.close() 
      }

      return browser as puppeteer.Browser & AsyncDisposable & Disposable
    })
  }

  return await BROWSER
}

export async function connect_to_browser(): Promise<puppeteer.Browser & AsyncDisposable & Disposable> {
  if (!fs.existsSync(PATH_BROWSER_SOCKET)) {
    throw new Error('Browser already running')
  }

  if (!BROWSER) {
    BROWSER = puppeteer.connect({
      browserWSEndpoint: fs.readFileSync(PATH_BROWSER_SOCKET, 'utf8')
    }).then(browser => {
      // @ts-expect-error
      browser[Symbol.asyncDispose] = async () => {
        await browser.disconnect() 
      }
    
      // @ts-expect-error
      browser[Symbol.dispose] = async () => {
        await browser.disconnect() 
      }

      return browser as puppeteer.Browser & AsyncDisposable & Disposable
    })
  }

  return await BROWSER
}
