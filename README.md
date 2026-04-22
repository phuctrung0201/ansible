# Overview
This repository stores personal machine setup for:
 - [oh-my-zsh](https://ohmyz.sh/)
 - [Ghostty](https://ghostty.org/)
 - [AeroSpace](https://github.com/nikitabobko/AeroSpace)
 - [Neovim](https://neovim.io/)
 - [mise](https://mise.jdx.dev/)

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
mise run install -- --tags "ohmyzsh"
```
