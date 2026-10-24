# Overview

Personal **macOS** machine setup for:

- [Fish](https://fishshell.com/) (default login shell — [Carapace](https://carapace.sh/) completions, [zoxide](https://github.com/ajeetdsouza/zoxide), vi mode, mise integration)
- [mise](https://mise.jdx.dev/)
- [Neovim](https://neovim.io/) with
  [obsidian.nvim](https://github.com/obsidian-nvim/obsidian.nvim) integration
- [Obsidian](https://obsidian.md/) (default theme with a managed Catppuccin Mocha CSS override)
- [Paneru](https://github.com/karinushka/paneru) (Niri-style scrolling tiling WM with Lua-managed config)
- [Kitty](https://sw.kovidgoyal.net/kitty/) (terminal emulator with Catppuccin Mocha and Fish integration)
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

Kitty uses Option as Alt and shares the same Catppuccin palette as Neovim/Fish.
Paneru provides:

- Option+H/J/K/L to focus windows and Option+Shift+H/J/K/L to move them
- Option+LeftBracket/Option+RightBracket to jump to first/last column
- Option+Shift+LeftBracket/Option+Shift+RightBracket to move a column to first/last
- Option+Semicolon to stack and Option+Shift+Semicolon to unstack
- Option+Quote to toggle full width and Option+Shift+Quote to toggle floating
- Option+Slash to balance columns
- Option+Shift+Slash to equalize the focused stack
- Option+,/Option+. to cycle column width
- Option+Shift+,/Option+Shift+. to cycle window height
- Option+Q to quit and Option+R to restart Paneru

Run either migrated role independently with `--tags kitty` or
`--tags paneru`.

# Default login shell

To use fish as your default login shell, register the Homebrew binary in
`/etc/shells` and run `chsh` (requires sudo):

```sh
FISH="/opt/homebrew/bin/fish"
grep -qxF "$FISH" /etc/shells || echo "$FISH" | sudo tee -a /etc/shells
chsh -s "$FISH"
```

Log out and back in (or open a new terminal) for the change to take effect.
