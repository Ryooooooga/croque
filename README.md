# Croq

[![](https://github.com/Ryooooooga/croq/workflows/Build/badge.svg)](https://github.com/Ryooooooga/croq/actions)
[![](https://badgen.net/crates/v/croq)](https://crates.io/crates/croq)

Fast and customizable shell prompt.

![](docs/screenshot.png)

## Optional Requirements

- [gh](https://github.com/cli/cli)
- [glab](https://docs.gitlab.com/ee/integration/glab)

## Installation

### From source

```sh
cargo install croq
```

### From Homebrew

```sh
brew install ryooooooga/tap/croq
```

### From precompiled binary

[releases](https://github.com/Ryooooooga/croq/releases)

### Bash

```bash
# ~/.bashrc
eval "$(croq init bash)"
```

### Zsh

```zsh
# ~/.zshrc
eval "$(croq init zsh)"
```

#### Using [Zinit](https://github.com/zdharma-continuum/zinit)

```zsh
# .zshrc
zinit light-mode from'gh-r' as'program' for \
    atclone'./croq init zsh >croq.zsh; zcompile croq.zsh' atpull'%atclone' \
    src'croq.zsh' \
    @'Ryooooooga/croq'
```

### Fish

```fish
# ~/.config/fish/config.fish
croq init fish | source
```

## Configuration

```sh
$ croq config > ~/.config/croq/config.yaml
```
