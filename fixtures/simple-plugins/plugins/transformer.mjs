import { Transformer } from '@alshdavid/mach'

export default new Transformer({
  async transform({ asset, config }) {
    console.log(asset)
    console.log({
      file_path: asset.file_path,
      kind: asset.kind,
      message: "from js transformer", 
      code_length: (await asset.get_code()).length,
    })
  }
})
