import path from 'node:path'
import * as url from 'node:url'

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))
export const FIXTURES = (...segments: string[]) =>
  path.resolve(__dirname, '..', '..', 'fixtures', ...segments)
export const FIXTURES_PATH_FN =
  (...base_segments: string[]) =>
  (...segments: string[]) =>
    path.resolve(
      __dirname,
      '..',
      '..',
      'fixtures',
      ...base_segments,
      ...segments,
    )
