# 🌏️ Mach 🚀

## The Bundler from Down Under

Mach is a zero configuration bundler for web applications.

### Installation

[Skip to installing Mach binary](#install-mach-binary)

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

### Plugins

Mach has a rich plugin system allowing plugin authors to add middleware into various stages of the bundling process with support for plugins compiled to WASM and slower plugins written in JavaScript.

### Install Mach Binary

#### MacOS

```bash
mkdir -p $HOME/.local/mach
curl -L --url https://github.com/alshdavid/mach/releases/latest/download/macos-arm64.tar.gz | tar -xvzf - -C $HOME/.local/mach
echo "\nexport \PATH=\$PATH:\$HOME/.local/mach\n" >> $HOME/.zshrc
source $HOME/.zshrc
```

##### Updating

```bash
rm -rf $HOME/.local/mach
mkdir -p $HOME/.local/mach
curl -L --url https://github.com/alshdavid/mach/releases/latest/download/macos-arm64.tar.gz | tar -xvzf - -C $HOME/.local/mach
```

#### Linux

```bash
mkdir -p $HOME/.local/mach
curl -L --url https://github.com/alshdavid/mach/releases/latest/download/linux-amd64.tar.gz | tar -xvzf - -C $HOME/.local/mach
echo "\nexport \PATH=\$PATH:\$HOME/.local/mach\n" >> $HOME/.zshrc
source $HOME/.zshrc
```

##### Updating

```bash
rm -rf $HOME/.local/mach
mkdir -p $HOME/.local/mach
curl -L --url https://github.com/alshdavid/mach/releases/latest/download/linux-amd64.tar.gz | tar -xvzf - -C $HOME/.local/mach
```

#### Windows

I'm not good at PowerShell - follow the same steps as the Linux/MacOS scripts

### Credit

Mach is heavily inspired by Parcel, deriving learnings, approaches and forking code from the project.
