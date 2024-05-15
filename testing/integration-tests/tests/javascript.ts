import {test, before, after, describe} from 'node:test';
import assert from 'node:assert';
import * as puppeteer from 'puppeteer-core';
import { connect_to_browser } from '../utils/browser';

describe('javascript', { concurrency: true }, () => {
  let browser: puppeteer.Browser

  before(async () => {
    browser = await connect_to_browser()
  })

  after(async () => {
    await browser.disconnect()
  })

  test('synchronous passing test', async (t) => {
    console.log(browser)
    await new Promise<void>(res => setTimeout(res, 5000))

    assert.strictEqual(1, 1);
  });

  test('synchronous passing test', async (t) => {
    console.log(browser)
    await new Promise<void>(res => setTimeout(res, 5000))
    
    assert.strictEqual(1, 1);
  });
})