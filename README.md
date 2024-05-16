<h1 align="center">🌏️ <img height="40px" align="center" src="./.docs/assets/logo.svg"></img> 🚀</h1>

<h3 align="center">Zero Configuration. Zero Dependencies. Fast AF</h3>

<p align="center">
  <img height="20px" src="./.docs/assets/prerelease.svg"></img>
</p>

<p align="center"><i>
  Mach is a super fast multi-threaded bundler written in Rust that puts an emphasis on<br>
  developer experience and the runtime performance of the compiled application.<br>
  <br>
  Mach is heavily inspired by the <a href="https://parceljs.org/">Parcel bundler</a>
</i></p>

<p align="center">
  <a href=".docs/CONTRIBUTING.md">Contributing Guidelines</a>
  .
  <a href="https://github.com/alshdavid/mach/issues">Submit an Issue</a>
  .
  <a href="https://github.com/alshdavid/mach/discussions">Ask a Question</a>
  .
  <a href="BLOG.md">Blog</a>
</p>

<p align="center">
  <img src="https://img.shields.io/npm/v/@alshdavid/mach">
  <img src="https://img.shields.io/npm/dm/@alshdavid/mach.svg">
  <img src="https://img.shields.io/badge/install_dependencies-0-green">
</p>

---

## Installation

You can install Mach from npm or get the latest binary from the github [releases](https://github.com/alshdavid/mach/releases/latest)

```bash
npm install @alshdavid/mach
npx mach version
```

## Usage

```bash
$ mach build ./src/index.html
$ mach dev ./src/index.html #todo
```

## Programmatic Usage

```javascript
import { Mach } from '@alshdavid/mach'

// Create a Mach instance
const mach = new Mach()

// Listen to build events
mach.subscribe('build_event', event => console.log(event))

// Build a target
const report = await mach.build({
  projectRoot: process.cwd(),
  outFolder: 'dist',
  entries: ['src/index.js']
})
```

## Supported Types

Mach comes preconfigured with sensible defaults and does not need configuration. Mach ships with built-in support for the most common source files in web development.

- TypeScript
- JavaScript
- JSX and TSX
- CSS
- HTML
- Images (todo)

## Plugins

Mach supports plugins that share the ideas and API of [Parcel Plugins](https://parceljs.org/features/plugins) for cases where bundling must be customized.

- [Resolver](https://parceljs.org/plugin-system/resolver/) _partial support_
- [Transformer](https://parceljs.org/plugin-system/transformer/) _partial support_
- [Reporter](https://parceljs.org/plugin-system/reporter/) _todo_
- [Namer](https://parceljs.org/plugin-system/namer/) _todo_

Plugins can be written in:
- [JavaScript](./.docs/PLUGINS_NODEJS.md) _in progress_
- [Rust (Dynamically Loaded)](./.docs/PLUGINS_RUST.md) _todo_
- [Wasm](./.docs/PLUGINS_WASM.md) _todo_

### JavaScript Plugins

Import the plugin API from the `@alshdavid/mach` npm package

```javascript
import { Transformer } from '@alshdavid/mach'
```

#### Performance

It goes without saying that JavaScript plugins are not as fast plugins written in Rust. While JavaScript itself is fast, there is additional overhead associated with talking to Nodejs from an external process.

To mitigate this, Mach uses low level APIs provided by the operating system to share memory and communicate with a Nodejs child process/workers.

On an M1 MacBook Pro; In addition to the time taken for the plugin code to run, a project with 1000 files will see 100ms added to the build time for each JavaScript plugin added.

This can add up quickly, especially when considering the endless void that is node_modules. It's recommended that performance sensitive projects migrate JavaScript plugins to Rust.

## Benchmark

Below is a build of the three-js source code multiplied 100 times and reexported from a single entry point. The benchmark was run on an M1 MacBook Pro with optimizations/minification turned off.

Mach is still under development and has lots of known opportunities for further build time improvements 🙂

<p align="center">
  <img src="./.docs/assets/benchmarks/benchmark-2024-05-14.png">
</p>

## Remaining work

The goal of Mach 1 will be a super fast production ready bundler with plugin support with some features remaining to be added (for instance - incremental bundling, or caching). 

The order of these may change and some may be pushed back to Mach 2

**🧩 Prerelease [Flyer](https://en.wikipedia.org/wiki/Wright_Flyer)**
- Plugin support
- Minification

**🧩 Prerelease [Red Baron](https://en.wikipedia.org/wiki/Fokker_Dr.I)**
- Source Maps

**🧩 Prerelease [Spitfire](https://en.wikipedia.org/wiki/Supermarine_Spitfire)**
- Watch mode / Auto-recompilation 
- Development server
- Hot reload

**🧩 Prerelease [Mustang](https://en.wikipedia.org/wiki/North_American_P-51_Mustang)**
- Bundle splitting (help wanted 🚩)

**🧩 Prerelease [Shooting Star](https://en.wikipedia.org/wiki/Lockheed_P-80_Shooting_Star)**
- Incremental Bundling for Development

**👀 Release Candidate [X-1](https://en.wikipedia.org/wiki/Bell_X-1)**
- TBD

**🛩️ Mach 1 - Codenamed [Concorde](https://en.wikipedia.org/wiki/Concorde)**
- TBD

## Special Thanks

<img align="right" height="40px" src="./.docs/assets/logo-parcel.svg" />
Mach is heavily inspired by Parcel. It derives learnings, approaches and adapts code directly from the project.<br>
<a href="https://parceljs.org/">Check it out here</a><br>

---

<img align="right" height="50px" src="./.docs/assets/logo-atlassian.svg" />
Special thanks to Atlassian for supporting my independent development
of this project during my employment with them.<br>
<a href="https://www.atlassian.com/">Learn about Atlassian</a>

---

<img align="right" height="80px" src="./.docs/assets/logo-rust-discord.png" />
Special thanks to the Rust Community Discord, an amazing community of talented engineers who were <br>welcoming and always happy to help out.<br>
<a href="https://github.com/rust-community-discord">Join the Discord Here</a>

## Benchmarks

The benchmark takes the three-js source code, copies it 50 times, imports the 50 copies from a single entrypoint and measures the time to build.

```javascript
import * as copy_1 from './copy_1/Three.js'; window.copy_1 = copy_1;
import * as copy_2 from './copy_2/Three.js'; window.copy_2 = copy_2;
import * as copy_3 from './copy_3/Three.js'; window.copy_3 = copy_3;
// ... and so on
```

The hardware I am using is a desktop AMD 7950x with 16 cores and the builds are using 16 threads.


<p align="center">
  <img align="center" width="100%" src="./.docs/assets/benchmarks/bench-2024-03-29.jpg">
  <br>
  <i>Build Time (lower is better)</i>
</p>