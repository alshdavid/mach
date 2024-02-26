<h1 align="center">🌏️ Mach 🚀</h1>

<h3 align="center"><i>
  Zero Configuration Bundler For The Modern Web
</i></h3>

<p align="center"><i>
  Mach is a super fast bundler written in Rust that is<br>
  inspired by the <a href="https://parceljs.org/">Parcel bundler</a>
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
```

## Usage

[Read more here](.docs/usage.md)

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

<br>

<h3 align="center">Special Thanks</h3>
<br>

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
