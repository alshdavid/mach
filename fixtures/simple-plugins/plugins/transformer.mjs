import { Transformer } from '@alshdavid/mach'

export default new Transformer({
  async transform({ asset, config }) {
    console.log("from js transformer", (await asset.get_code()).length)
  }
})
