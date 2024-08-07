import { Transformer } from '@alshdavid/mach'

export default new Transformer({
  loadConfig() {
    return 'TextTransformerConfig'
  },
  async transform({ asset, config }) {
    // Testing if loadConfig works
    if (typeof config !== 'string' || config !== 'TextTransformerConfig') {
      throw new Error('Did not load config')
    }

    const contents = await asset.getCode()

    asset.setCode(`
      export default "${contents}"
    `)

    asset.type = 'js'

    return asset
  },
})
