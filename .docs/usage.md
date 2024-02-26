## Usage

```bash
$ mach build ./src/index.js
> Build Success

$ ls ./dist
> index.js
```

## CLI Arguments

|Arguments|Description|
|-|-|
|`-z` `--optimize`| Optimize the output, minifying, tree shaking and so on |
| `-t` `--threads` | Set the number of threads to use for building |
| `--node-workers` | Set the number of worker threads Node will spawn to execute JS plugins with |

## Plugins

Mach aims to support part of the [Parcel Plugin API](https://parceljs.org/features/plugins/)

Currently supported:
- [Resolver Plugin (partial support)](https://parceljs.org/plugin-system/resolver)
- [Transformer Plugin (partial support)](https://parceljs.org/plugin-system/transformer)

#### Machrc

Create a `.machrc` file in the root of your project. The configuration format follows the [Parcel configuration format](https://parceljs.org/features/plugins/#.parcelrc) with a twist.

```json
{
  "resolvers": [
    "node:some-resolver-plugin",
    "mach:resolver"
  ],
  "transformers": {
    "*.{js,mjs,jsm,jsx,es6,cjs,ts,tsx}": [
      "node:./plugins/transformer.mjs",
      "mach:transformers/javascript"
    ]
  }
}
```

Entries must specify the "adapter" used to resolve & evaluate the plugin. 

- The `mach:*` adapter uses the builtin plugins
- The `node:*` adapter uses `node` specified in your `PATH` to execute plugins.

In future I will add support for dynamic Rust plugins and may eventually add support for WASM plugins if there is a good case for it.
