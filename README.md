# Overview

Personal **macOS** machine setup for:

- [Fish](https://fishshell.com/) (default login shell — [Carapace](https://carapace.sh/) completions, [zoxide](https://github.com/ajeetdsouza/zoxide), vi mode, mise integration)
- [mise](https://mise.jdx.dev/)
- [Neovim](https://neovim.io/) with
  [obsidian.nvim](https://github.com/obsidian-nvim/obsidian.nvim) integration
- [Obsidian](https://obsidian.md/) (default theme with a managed Catppuccin Mocha CSS override)
- [Rift](https://github.com/acsandmann/rift) (tiling WM, configured with the niri-style scrolling layout)
- [JankyBorders](https://github.com/FelixKratz/JankyBorders) (active-window border in the theme peach color)
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
Rift (scrolling layout) provides:

- Option+H/J/K/L to focus windows and Option+Shift+H/J/K/L to move them
- Option+Tab/Option+Shift+Tab to focus the next/previous window
- Option+Semicolon to consume or expel the window to the right and Option+Shift+Semicolon to the left
- Option+Quote to show Rift's mission control for the current workspace and Option+Shift+Quote for all
- Option+Backslash to toggle full width (within gaps) and Option+Shift+Backslash to toggle floating
- Option+[/Option+] to cycle column width presets backward/forward
- Option+Shift+[/Option+Shift+] to shrink/grow window height
- Option+R to reload config and Option+Q to save layout and quit Rift
- Option+Space to open the Kitty quick access terminal

Run either migrated role independently with `--tags kitty` or
`--tags rift`.

# Default login shell

To use fish as your default login shell, register the Homebrew binary in
`/etc/shells` and run `chsh` (requires sudo):

```sh
FISH="/opt/homebrew/bin/fish"
grep -qxF "$FISH" /etc/shells || echo "$FISH" | sudo tee -a /etc/shells
chsh -s "$FISH"
```

Log out and back in (or open a new terminal) for the change to take effect.
