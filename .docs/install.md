# Installation

You can install Mach from npm or directly as a [binary](#binary)

## NPM

```bash
npm install @alshdavid/mach
```

## Binary

### MacOS (Install and Update)

```bash
rm -rf $HOME/.local/mach
mkdir -p $HOME/.local/mach
curl -L --url https://github.com/alshdavid/mach/releases/latest/download/macos-arm64.tar.xz | tar -xvzf - -C $HOME/.local/mach
export PATH=$HOME/.local/mach
```

Add the following to your `~/.zshrc` and/or `~/.bashrc`
```bash
export PATH=$HOME/.local/mach:$PATH
```

### Linux (Install and Update)

```bash
rm -rf $HOME/.local/mach
mkdir -p $HOME/.local/mach
curl -L --url https://github.com/alshdavid/mach/releases/latest/download/linux-amd64.tar.xz | tar -xvzf - -C $HOME/.local/mach
```

Add the following to your `~/.zshrc`
```bash
export PATH=$HOME/.local/mach:$PATH
```

### Windows (Install and Update)

```
I'm not good at Windows scripting, install instructions coming soon
```
