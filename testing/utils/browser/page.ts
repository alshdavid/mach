import * as puppeteer from 'puppeteer-core';
import { connect_to_browser } from './browser';

let PAGES = 0

export async function create_page(url?: string): Promise<puppeteer.Page & AsyncDisposable & Disposable> {
  PAGES += 1
  const browser = await connect_to_browser()
  const page = await browser.newPage()

  // @ts-expect-error
  page[Symbol.asyncDispose] = async () => {
    await page.close()
    PAGES -= 1
    if (PAGES === 0) await browser.disconnect()
  }

  // @ts-expect-error
  page[Symbol.dispose] = async () => {
    await page.close() 
    PAGES -= 1
    if (PAGES === 0) await browser.disconnect()
  }

  if (url) {
    await page.goto(url)
  }

  return page as puppeteer.Page & AsyncDisposable & Disposable
}