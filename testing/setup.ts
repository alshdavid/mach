import * as foo from '@alshdavid/mach-os-arch'

console.log(foo.Mach)


// import '@shigen/polyfill-symbol-dispose'
// import * as reporter from 'node:test/reporters'
// import { run } from 'node:test'
// import * as path from 'node:path'
// import * as fs from 'node:fs'
// import * as url from 'node:url'
// import * as puppeteer from 'puppeteer-core'
// import { finished } from 'node:stream'
// import { CHROME_EXECUTABLE_PATH } from './utils/browser/executable.js'

// const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

// void (async function () {
//   const browser = await puppeteer.launch({
//     executablePath: CHROME_EXECUTABLE_PATH,
//     headless: true,
//     devtools: true,
//     ignoreHTTPSErrors: true,
//     args: [
//       '--no-sandbox',
//       '--disable-setuid-sandbox',
//       '--disable-sync',
//       '--ignore-certificate-errors',
//       '--disable-gpu',
//     ],
//   })

//   process.env.PUPPETEER_WS_ENDPOINT = browser.wsEndpoint()

//   let files = []
//   if (process.argv.slice(2).length) {
//     for (const option of process.argv.slice(2)) {
//       if (option.startsWith('.')) {
//         files.push(path.join(process.cwd(), option))
//       } else if (option.startsWith(path.sep)) {
//         files.push(option)
//       } else {
//         files.push(path.join(__dirname, 'tests', option))
//       }
//     }
//   } else {
//     files = fs
//       .readdirSync(path.join(__dirname, 'tests'))
//       .filter((test) => !test.startsWith('_'))
//       .map((test) => path.join(__dirname, 'tests', test))
//   }

//   const test_stream = run({
//     files,
//     concurrency: true,
//   })
//     .on('test:fail', () => {
//       process.exitCode = 1
//     })
//     .compose(new reporter.spec())

//   test_stream.pipe(process.stdout)
//   await new Promise((res) => finished(test_stream, res))
//   await browser.close()
// })()
