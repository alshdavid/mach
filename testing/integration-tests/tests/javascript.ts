import {test, before, after, describe} from 'node:test';
import assert from 'node:assert';
import * as puppeteer from 'puppeteer-core';
import { FIXTURES, build_mach, connect_to_browser } from '../utils/browser';

describe('javascript', { concurrency: true }, () => {
  let browser: puppeteer.Browser

  before(async () => {
    browser = await connect_to_browser()
  })

  after(async () => {
    await browser.disconnect()
  })

  test('synchronous passing test', async (t) => {
    const result = await build_mach({
      cwd: FIXTURES('commonjs'),
      entries: ['src/index.js']
    })
    console.log(result)
  });

})