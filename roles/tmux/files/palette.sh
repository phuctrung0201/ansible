#!/usr/bin/env bash
# tmux command palette — fuzzy-pick a useful tmux command
# Invoked from a `display-popup -E` binding; runs the chosen command via tmux.

set -euo pipefail

# Nerd-font glyph (nf-fa-terminal) — printf'd to keep the source pure ASCII
ICON=$(printf '\xef\x84\xa0')

# Catppuccin Mocha overlay0 — comment-like dim grey for the hint line.
HINT=$'\e[38;2;108;112;134m'
RESET=$'\e[0m'

FZF_OPTS=(--height=100% --layout=reverse --border=rounded)

# Format per line: <label>\t<hint>\t<command>
# Default: <command> is a tmux subcommand (we run `tmux <cmd>`).
# Lines starting with `!` run as raw shell — external scripts or $(...) substitution.
static=$(cat <<'EOF'
Rename session (folder)	tmux	!tmux rename-session "$(basename "$(tmux display-message -p '#{pane_current_path}')")"
Kill window	tmux	kill-window
Kill pane	tmux	kill-pane
Break to new window	tmux	break-pane
Swap pane left	tmux	swap-pane -s '{left-of}'
Swap pane down	tmux	swap-pane -s '{down-of}'
Swap pane up	tmux	swap-pane -s '{up-of}'
Swap pane right	tmux	swap-pane -s '{right-of}'
Even horizontal	tmux	select-layout even-horizontal
Even vertical	tmux	select-layout even-vertical
Main horizontal	tmux	select-layout main-horizontal
Main vertical	tmux	select-layout main-vertical
Tiled	tmux	select-layout tiled
Copy password	lpass	!~/.config/tmux/lpass.sh password
Copy username	lpass	!~/.config/tmux/lpass.sh username
Add credential	lpass	!~/.config/tmux/lpass.sh add
Generate password	lpass	!~/.config/tmux/lpass.sh generate
EOF
)

# Dynamic: one entry per existing session.
sessions=$(tmux list-sessions -F '#{session_name}' 2>/dev/null \
  | awk -v OFS='\t' '{print "Switch to " $0, "tmux", "switch-client -t \"" $0 "\""}')

# Render <label>\n<colored-hint>\t<cmd> per entry (NUL-separated) so fzf shows the
# hint on a line below the label. `sort -f` alphabetizes before formatting.
selection=$(printf '%s\n%s\n' "$static" "$sessions" \
  | sort -f \
  | while IFS=$'\t' read -r label hint cmd; do
      [ -z "$label" ] && continue
      printf '%s\n%s%s%s\t%s\0' "$label" "$HINT" "$hint" "$RESET" "$cmd"
    done \
  | fzf "${FZF_OPTS[@]}" --read0 --ansi --with-nth=1 --accept-nth=2 --delimiter=$'\t' --no-multi \
      --prompt="❯ " --border-label=" $ICON tmux palette " --border-label-pos=3) || exit 0

[ -z "$selection" ] && exit 0

if [[ "$selection" == !* ]]; then
  eval "${selection#!}"
else
  eval "tmux $selection"
fi
