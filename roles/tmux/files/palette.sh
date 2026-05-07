#!/usr/bin/env bash
# tmux command palette — fuzzy-pick a useful tmux command
# Invoked from a `display-popup -E` binding; runs the chosen command via tmux.

set -euo pipefail

# Nerd-font glyph (nf-fa-terminal) — printf'd to keep the source pure ASCII
ICON=$(printf '\xef\x84\xa0')

# Format per line: <label>\t<command>
# Default: command is a tmux subcommand (we run `tmux <cmd>`).
# Lines starting with `!` are run as raw shell instead (for $(...) substitution).
# `\;` survives eval as a literal `;` arg, which tmux treats as a command separator.
static=$(cat <<'EOF'
rename session to basename	!tmux rename-session "$(basename "$(tmux display-message -p '#{pane_current_path}')")"
rename window to agent	rename-window agent
rename window to markserv	rename-window markserv
kill other sessions	confirm-before -p "kill all other sessions? (y/n)" "kill-session -a"
kill other windows	confirm-before -p "kill all other windows? (y/n)" "kill-window -a"
kill other panes	confirm-before -p "kill all other panes? (y/n)" "kill-pane -a"
lastpass	!~/.config/tmux/lpass.sh
EOF
)

# Dynamic: one entry per existing session and per window in current session.
sessions=$(tmux list-sessions -F '#{session_name}' 2>/dev/null \
  | awk -v OFS='\t' '{print "switch to " $0 " session", "switch-client -t \"" $0 "\""}')

windows=$(tmux list-windows -F $'#{window_index}\t#{window_name}' 2>/dev/null \
  | awk -F'\t' -v OFS='\t' '{print "switch to " $2 " window", "select-window -t " $1}')

selection=$(printf '%s\n%s\n%s\n' "$static" "$sessions" "$windows" | awk 'NF' | \
  fzf --with-nth=1 \
      --delimiter=$'\t' \
      --prompt="$ICON command ❯ " \
      --header='' \
      --no-multi \
      --height=100% \
      --layout=reverse) || exit 0

cmd=$(printf '%s' "$selection" | cut -f2-)
[ -z "$cmd" ] && exit 0

if [[ "$cmd" == !* ]]; then
  eval "${cmd#!}"
else
  eval "tmux $cmd"
fi
