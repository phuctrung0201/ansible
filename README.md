# Overview

Personal **macOS** machine setup for:

- [Fish](https://fishshell.com/) (default login shell — [Carapace](https://carapace.sh/) completions, [zoxide](https://github.com/ajeetdsouza/zoxide), vi mode, mise integration)
- [mise](https://mise.jdx.dev/)
- [Neovim](https://neovim.io/) with
  [obsidian.nvim](https://github.com/obsidian-nvim/obsidian.nvim) integration
- [Obsidian](https://obsidian.md/) (default theme with a managed Catppuccin Mocha CSS override)
- [OmniWM](https://github.com/BarutSRB/OmniWM) (Niri-style sliding tiling WM with overview and quake terminal)
- [Ghostty](https://ghostty.org/) (terminal emulator with Catppuccin Mocha and Fish integration)
- [LastPass CLI](https://github.com/lastpass/lastpass-cli) (`flpass` fuzzy picker via [fzf](https://github.com/junegunn/fzf))

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

Ghostty maps Option+Shift+Enter to write the current screen to a temporary file
and copy its path. OmniWM keeps native macOS Spaces and provides:

- Option+H/L to focus windows and Option+J/K to focus a window or workspace
- Option+Shift+H/L to move windows and Option+Shift+J/K to move a window or workspace
- Option+Semicolon/Option+Shift+Semicolon to move a column left/right
- Option+Quote to raise floating windows and Option+Shift+Quote to toggle floating
- Option+Slash to toggle full width within the configured gaps
- Option+Shift+Slash to balance window sizes
- Option+,/Option+. to cycle column width
- Option+Shift+,/Option+Shift+. to cycle window height
- Option+Grave to open the OmniWM menu anywhere
- Option+Space for the centred quake terminal

Run either migrated role independently with `--tags ghostty` or
`--tags omniwm`.

# Default login shell

To use fish as your default login shell, register the Homebrew binary in
`/etc/shells` and run `chsh` (requires sudo):

```sh
FISH="$(brew --prefix)/bin/fish"
grep -qxF "$FISH" /etc/shells || echo "$FISH" | sudo tee -a /etc/shells
chsh -s "$FISH"
```

Log out and back in (or open a new terminal) for the change to take effect.
