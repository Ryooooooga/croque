# Croque

[![](https://github.com/Ryooooooga/croque/workflows/Build/badge.svg)](https://github.com/Ryooooooga/croque/actions)
[![](https://badgen.net/crates/v/croque)](https://crates.io/crates/croque)

Fast and customizable shell prompt.

## Installation

### From source

```sh
cargo install croque
```

### From Homebrew

```sh
brew install ryooooooga/tap/croque
```

### From precompiled binary

[releases](https://github.com/Ryooooooga/croque/releases)

### Bash

```bash
# ~/.bashrc
eval "$(croque init bash)"
```

### Zsh

```zsh
# ~/.zshrc
eval "$(croque init zsh)"
```

optional requirements: [zsh-async](https://github.com/mafredri/zsh-async)

### Fish

```fish
# ~/.config/fish/config.fish
croque init fish | source
```

## Configuration

```sh
$ croque config > ~/.config/croque/config.yaml
```
