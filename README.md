# 🌏️ Mach 🚀

## The Bundler from Down Under

Mach is a zero configuration bundler for web applications.

### Current Status

#### 25 Feb 2024

The latest branch `development` reworks a large portion of the bundler internals, adding support for JS plugins, changing the browser runtime code, and adds many other features.

It will be merged as soon as it produces a working bundle.

### Installation

Right now Mach is distributed as a binary. In the future I'll look at publishing it on the various package managers.

[Installing Mach binary](.docs/install.md)

```
## coming soon
npm install @alshdavid/mach
```

### Usage

```
$ mach build ./src/index.js
> Build Success

$ ls ./dist
> index.js
```

#### Options

|Arguments|Description|
|-|-|
|`-z` `--optimize`| Optimize the output, minifying, tree shaking and so on |
| `-t` `--threads` | Set the number of threads to use for building |
| `--node-workers` | Set the number of worker threads Node will spawn to execute JS plugins with |

### Plugins

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

<br>

---

<h2 align="center">Special Thanks</h2>

<p align="center">
  <img height="30px" src="./.docs/assets/logo-parcel.svg" />
  <br> 
  Mach is heavily inspired by Parcel.<br>
  It derives learnings, approaches and adapts code directly from the project.<br>
  <br>
  <a href="https://parceljs.org/">Check it out here</a><br>
</p>

---

<p align="center">
  <img height="50px" src="./.docs/assets/logo-atlassian.svg" />
  <br> 
  Special thanks to Atlassian for supporting my independent development<br>
  of this project during my employment with them.<br>
  <br>
  <a href="https://www.atlassian.com/">Learn about Atlassian</a><br>
</p>

---

<p align="center">
  <img height="80px" src="./.docs/assets/logo-rust-discord.png" />
  <br>
  Special thanks to the Rust Community Discord<br>
  An amazing community of people who were always available to help me<br>
  with any questions I had, no matter how dumb.<br>
  <br>
  <a href="https://github.com/rust-community-discord">Join here</a><br>
</p>
