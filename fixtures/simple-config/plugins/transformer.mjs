import { Transformer } from '@alshdavid/mach'

export default new Transformer({
  transform({ asset }) {
    console.error(asset.filePath)
    console.error(asset.code)
    return [asset]
  }
})
