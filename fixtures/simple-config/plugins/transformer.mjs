import { Transformer } from '@alshdavid/mach'

export default new Transformer({
  transform({ asset }) {
    console.error(asset.filePath)
    console.error(asset.code)
    return [asset]
  }
})

/*
{ 
  "id": 1,
  "action":"plugin_load_transformer",
  "data": {
    "specifier": "/home/dalsh/Development/alshdavid/mach/fixtures/simple-config/plugins/transformer.mjs" 
  }
}

{ 
  "id": 2, 
  "action":"plugin_run_transform", 
  "data": { 
    "specifier": "/home/dalsh/Development/alshdavid/mach/fixtures/simple-config/plugins/transformer.mjs",
    "asset": { 
      "filePath": "foo" 
    } 
  }
}
*/