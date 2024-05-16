import * as http from 'node:http';
import { serve_static } from './content_type.js'
import * as puppeteer from 'puppeteer-core';

export type ClientContextOptions = {
  browser: puppeteer.Browser
  serve_path: string
}

export class ClientContext {
  #server
  #browser

  constructor(
    server: http.Server,
    browser: puppeteer.Browser,
  ) {
    this.#server = server
    this.#browser = browser
  }

  static async new(options: ClientContextOptions): Promise<ClientContext> {
    const server = http.createServer(serve_static(options.serve_path))
    await new Promise<void>(res => server.listen(0, "0.0.0.0", () => res()))
    return new ClientContext(server, options.browser)
  }

  // async newPage(): Promise<puppeteer.Page & AsyncDisposable & Disposable> {
  //   const page = await this.#browser.newPage()
    
  //   await page.goto(`http://localhost:${this.#server.address().port}`)

   

  //   return page as puppeteer.Page & AsyncDisposable & Disposable
  // }

  address() {
    // @ts-expect-error
    return `http://localhost:${this.#server.address().port}`
  }

  close() {
    this.#server.close()
  }
}
