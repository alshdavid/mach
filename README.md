<h1 align="center">🌏️ Mach 🚀</h1>

<h3 align="center">Zero Configuration Bundler For The Modern Web</h3>

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
npx mach --version
```

## Usage

[Read more here](.docs/usage.md)

#### Simple Build

```bash
$ mach build ./src/index.html
> Build Success

$ ls ./dist
> index.js
```

#### Dev Server

```bash
$ mach dev ./src/index.html
> Serving on http://localhost:4242
```

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
