<h1 align="center">🌏️ Mach 🚀 (Alpha Phase)</h1>

<h3 align="center">Zero Configuration Bundler For The Modern Web</h3>

<p align="center"><i>
  Mach is a super fast multi-threaded bundler written in Rust that is<br>
  inspired by the <a href="https://parceljs.org/">Parcel bundler</a> and has zero dependencies.
</i></p>

<p align="center">
  <a href=".docs/CONTRIBUTING.md">Contributing Guidelines</a>
  .
  <a href="https://github.com/alshdavid/mach/issues">Submit an Issue</a>
  .
  <a href="https://github.com/alshdavid/mach/discussions">Ask a Question</a>
</p>

<p align="center">
  <img src="https://img.shields.io/npm/v/@alshdavid/mach">
  <img src="https://img.shields.io/npm/dm/@alshdavid/mach.svg">
  <img src="https://img.shields.io/badge/install_dependencies-0-green">
</p>

---

## Installation

You can install Mach from npm or directly as a [binary](.docs/install.md)

```bash
npm install @alshdavid/mach
npx mach --version
```

## Usage

[Read more here](.docs/usage.md)

#### Simple Build

```bash
$ mach build ./src/index.html
> Build Success

$ ls ./dist
> index.html index.js index.css
```

#### Dev Server

```bash
$ mach dev ./src/index.html
> Serving on http://localhost:4242
```

## Plugins

Plugins are supported by an embedded Deno runtime with support for Node.js, TypeScript, etc
This is still under development

## Benchmark

The benchmark takes the three-js source code, copies it 50 times, imports the 50 copies from a single entrypoint and measures the time to build.

```javascript
import * as copy_1 from './copy_1/Three.js'; window.copy_1 = copy_1;
import * as copy_2 from './copy_2/Three.js'; window.copy_2 = copy_2;
import * as copy_3 from './copy_3/Three.js'; window.copy_3 = copy_3;
// ... and so on
```

The hardware I am using is a desktop AMD 7950x with 16 cores and the builds are using 16 threads.

**20th March 2024**

<p align="center">
  <img align="center" width="100%" src="./.docs/assets/benchmarks/bench-2024-03-20.jpg">
  <br>
  <i>Build Time (lower is better)</i>
</p>

As of the 20th March 2024, this is a benchmark of Mach verses other bundlers in a "no minify" build. 

Mach is still in the early phase of development so I haven't spent a lot of time optimizing it - for example currently Mach doesn't share AST between the build phases causing it to double up on the parsing work.

There is a lot of room for optimization here so the numbers are likely to get better as we go 🙂

I've been working on integrating Deno for plugins and improving Node.js support.

Todo there is still development/watch mode and implementing a better bundling algorithm.

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
