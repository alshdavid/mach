import {test, describe, before, after} from 'node:test';
import * as assert from 'node:assert';
import { BuildReport, Mach } from '@alshdavid/mach';
import { FIXTURES_FN } from '../utils/paths/index.js';
import { CHROME_EXECUTABLE_PATH, ClientContext } from '../utils/browser/index.js';
import { install_npm } from '../utils/npm/index.js';
import * as puppeteer from 'puppeteer-core';

const FIXTURE = FIXTURES_FN('web-html-ts-css')

describe('web-html-ts-css', { concurrency: true }, async () => {
  let browser: puppeteer.Browser
  let client: ClientContext
  let report: BuildReport

  before(async () => {
    await install_npm(FIXTURE())

    report = await Mach.build({
      projectRoot: FIXTURE(),
      entries: ['src/index.html']
    })

    browser = await puppeteer.launch({
      executablePath: CHROME_EXECUTABLE_PATH,
      headless: true,
      devtools: true,
      ignoreHTTPSErrors: true,
      args: [
        '--no-sandbox',
        '--disable-setuid-sandbox',
        '--disable-sync',
        '--ignore-certificate-errors',
        '--disable-gpu'
      ],
    })

    client = await ClientContext.new({ 
      browser,
      serve_path: FIXTURE('dist')
    })
  })

  after(async () => {
    await browser.close()
    client.close()
  })

  test('Should set exports correctly ', async (t) => {
    const page = await browser.newPage()
    await page.goto(client.address())

    const innerText = await page.evaluate(() => window.document.body.innerText)
    const backgroundColor = await page.evaluate(() => window.getComputedStyle(document.body).backgroundColor)

    assert.equal(innerText, 'Hello World')
    assert.equal(backgroundColor, 'rgb(224, 224, 224)')
    await page.close()
  });
})

