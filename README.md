# Overview
This repository stores personal machine setup for:
 - [Nushell](https://www.nushell.sh/) (default login shell, with [zoxide](https://github.com/ajeetdsouza/zoxide) integration)
 - [mise](https://mise.jdx.dev/)
 - [Neovim](https://neovim.io/)
 - [AeroSpace](https://github.com/nikitabobko/AeroSpace)
 - [Kitty](https://sw.kovidgoyal.net/kitty/)
 - [tmux](https://github.com/tmux/tmux)

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
mise run install -- --tags "nushell"
```

Setting Nushell as the default login shell registers it in `/etc/shells` and runs
`chsh`, which require sudo. Run that part explicitly with `-K` (asks for your password):

```sh
mise run install -- --tags "chsh" -K
```
