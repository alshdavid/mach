# Installation

Right now Mach is distributed as a binary. 

In the future I'll look at publishing it on the various package managers.

## NPM (coming soon)

```bash
npm install @alshdavid/mach
```

## Binary

### MacOS (Install and Update)

```bash
rm -rf $HOME/.local/mach
mkdir -p $HOME/.local/mach
curl -L --url https://github.com/alshdavid/mach/releases/latest/download/macos-arm64.tar.gz | tar -xvzf - -C $HOME/.local/mach
export PATH=$HOME/.local/mach
```

Add the following to your `~/.zshrc`
```bash
export PATH=$HOME/.local/mach
```

### Linux (Install and Update)

```bash
rm -rf $HOME/.local/mach
mkdir -p $HOME/.local/mach
curl -L --url https://github.com/alshdavid/mach/releases/latest/download/linux-amd64.tar.gz | tar -xvzf - -C $HOME/.local/mach
```

Add the following to your `~/.zshrc`
```bash
export PATH=$HOME/.local/mach
```

### Windows (Install and Update)

```
I'm not good at Windows scripting, install instructions coming soon
```
