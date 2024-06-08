import * as fs from 'node:fs'
import * as path from 'node:path'
import * as url from 'node:url'

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

let new_index = ''

for (const filename of fs.readdirSync(path.join(__dirname, '..', 'types'))) {
  if (filename === 'index.d.ts') continue
  console.log(filename)
  new_index += `export type * from './${filename}'\n`
}

fs.rmSync(path.join(__dirname, '..', 'types', 'index.d.ts'), { force: true })
fs.writeFileSync(
  path.join(__dirname, '..', 'types', 'index.d.ts'),
  new_index,
  'utf8',
)
