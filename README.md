# Overview
This repository stores personal machine setup for:
 - [Zsh](https://www.zsh.org/) (default login shell — syntax highlighting, autosuggestions, [fzf](https://github.com/junegunn/fzf), [LastPass CLI](https://github.com/lastpass/lastpass-cli), [Starship](https://starship.rs/) prompt, vi keybindings, [zoxide](https://github.com/ajeetdsouza/zoxide))
 - [mise](https://mise.jdx.dev/)
 - [Neovim](https://neovim.io/)
 - [AeroSpace](https://github.com/nikitabobko/AeroSpace)
 - [Ghostty](https://ghostty.org/) (terminal emulator, integrated with tmux)
 - [tmux](https://github.com/tmux/tmux) (command palette via Alt+Space)

# Prerequisites

[mise](https://mise.jdx.dev/getting-started.html) is the only prerequisite.

# Runbook

Install tools and bootstrap Ansible:

```sh
mise install
```

Run the full setup:

```sh
mise run install
```

Run a specific role:

```sh
mise run install -- --tags "zsh"
```

Setting zsh as the default login shell registers it in `/etc/shells` and runs
`chsh`, which require sudo:

```sh
mise run install -- --tags "chsh" -e zsh_set_default_shell=true -K
```
