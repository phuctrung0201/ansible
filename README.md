# Overview

Personal **macOS** machine setup for:

- [Nushell](https://www.nushell.sh/) (default login shell — [Carapace](https://carapace.sh/) completions, [zoxide](https://github.com/ajeetdsouza/zoxide), vi mode, mise integration)
- [mise](https://mise.jdx.dev/)
- [Neovim](https://neovim.io/)
- [AeroSpace](https://github.com/nikitabobko/AeroSpace)
- [Ghostty](https://ghostty.org/) (terminal emulator, integrated with tmux)
- [tmux](https://github.com/tmux/tmux) (command palette via Alt+Space, [fzf](https://github.com/junegunn/fzf), [LastPass CLI](https://github.com/lastpass/lastpass-cli))

Nushell config is deployed to `~/Library/Application Support/nushell/` (the macOS default).

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

# Default login shell

To use nushell as your default login shell, register the Homebrew binary in
`/etc/shells` and run `chsh` (requires sudo):

```sh
NU="$(brew --prefix)/bin/nu"
grep -qxF "$NU" /etc/shells || echo "$NU" | sudo tee -a /etc/shells
chsh -s "$NU"
```

Log out and back in (or open a new terminal) for the change to take effect.
