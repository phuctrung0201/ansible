# Overview

Personal **macOS** machine setup for:

- [Fish](https://fishshell.com/) (default login shell — [Carapace](https://carapace.sh/) completions, [zoxide](https://github.com/ajeetdsouza/zoxide), vi mode, mise integration)
- [mise](https://mise.jdx.dev/)
- [Neovim](https://neovim.io/) with
  [obsidian.nvim](https://github.com/obsidian-nvim/obsidian.nvim) integration
- [Obsidian](https://obsidian.md/) (default theme with a managed Catppuccin Mocha CSS override)
- [OmniWM](https://github.com/BarutSRB/omniwm) (tiling WM — scrolling layout, with its built-in focus borders)
- [Ghostty](https://ghostty.org/) (terminal emulator, integrated with tmux)
- [tmux](https://github.com/tmux/tmux) (command palette via Alt+Space, [fzf](https://github.com/junegunn/fzf), [LastPass CLI](https://github.com/lastpass/lastpass-cli))

Fish config is deployed to `~/.config/fish/`.

# Prerequisites

- Python 3
- `pip`

# Runbook

Bootstrap Ansible and Galaxy collections:

```sh
pip install ansible ansible-lint
ansible-galaxy collection install -r requirements.yml
```

Run the full setup:

```sh
ansible-playbook main.yml
```

Run a specific role:

```sh
ansible-playbook main.yml --tags fish
```

The Obsidian role installs the app, deploys the Catppuccin Mocha snippet to
`~/wiki/.obsidian/snippets/`, and installs a local plugin that keeps the native
macOS traffic-light buttons hidden without revealing them on hover. Run it
independently with:

```sh
ansible-playbook main.yml --tags obsidian
```

# Default login shell

To use fish as your default login shell, register the Homebrew binary in
`/etc/shells` and run `chsh` (requires sudo):

```sh
FISH="$(brew --prefix)/bin/fish"
grep -qxF "$FISH" /etc/shells || echo "$FISH" | sudo tee -a /etc/shells
chsh -s "$FISH"
```

Log out and back in (or open a new terminal) for the change to take effect.
