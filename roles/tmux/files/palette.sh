#!/usr/bin/env bash
# tmux command palette — fuzzy-pick a useful tmux command
# Invoked from a `display-popup -E` binding; runs the chosen command via tmux.

set -euo pipefail

# Nerd-font glyph (nf-fa-terminal) — printf'd to keep the source pure ASCII
ICON=$(printf '\xef\x84\xa0')

# Format per line: <label>\t<hint>\t<command>
# <hint> is shown alongside the label as a secondary column (may be empty).
# Default: command is a tmux subcommand (we run `tmux <cmd>`).
# Lines starting with `!` are run as raw shell instead (for $(...) substitution).
# `\;` survives eval as a literal `;` arg, which tmux treats as a command separator.
static=$(cat <<'EOF'
Rename session to current folder	tmux	!tmux rename-session "$(basename "$(tmux display-message -p '#{pane_current_path}')")"
Rename window to agent	tmux	rename-window agent
Rename window to markserv	tmux	rename-window markserv
Close current window	tmux	kill-window
Close current pane	tmux	kill-pane
Close all other sessions	tmux	kill-session -a
Close all other windows	tmux	kill-window -a
Close all other panes	tmux	kill-pane -a
Detach session	tmux	detach-client
Break pane to window	tmux	break-pane
Move window to session	tmux	!t=$(tmux list-sessions -F '#{session_name}' | fzf --prompt="to session ❯ " --height=100% --layout=reverse --border=rounded) && [ -n "$t" ] && tmux move-window -t "$t"
Move pane to window	tmux	!cur=$(tmux display-message -p '#{window_index}'); t=$(tmux list-windows -F '#{window_index} (#{window_name})' | awk -v c="$cur" '$1 != c' | fzf --prompt="to window ❯ " --height=100% --layout=reverse --border=rounded | awk '{print $1}') && [ -n "$t" ] && tmux join-pane -t ":$t"
Copy password	lpass	!~/.config/tmux/lpass.sh password
Copy username	lpass	!~/.config/tmux/lpass.sh username
Add credential	lpass	!~/.config/tmux/lpass.sh add
Generate password	lpass	!~/.config/tmux/lpass.sh generate
EOF
)

# Dynamic: one entry per existing session and per window in current session.
sessions=$(tmux list-sessions -F '#{session_name}' 2>/dev/null \
  | awk -v OFS='\t' '{print "Switch to session " $0, "tmux", "switch-client -t \"" $0 "\""}')

windows=$(tmux list-windows -F $'#{window_index}\t#{window_name}' 2>/dev/null \
  | awk -F'\t' -v OFS='\t' '{print "Switch to window " $2, "tmux", "select-window -t " $1}')

# Catppuccin Mocha overlay0 — comment-like dim grey for the hint column.
HINT_COLOR=$'\e[38;2;108;112;134m'
RESET=$'\e[0m'

# Collapse <label>\t<hint>\t<cmd> into "<label>  <colored-hint>\t<cmd>" so fzf
# shows the hint as a tight inline suffix instead of a tab-stop-wide gap.
# `sort -f` orders entries alphabetically by label (case-insensitive).
selection=$(printf '%s\n%s\n%s\n' "$static" "$sessions" "$windows" \
  | awk -F'\t' -v D="$HINT_COLOR" -v R="$RESET" '
      NF { printf "%s  %s%s%s\t%s\n", $1, D, $2, R, $3 }' \
  | sort -f \
  | fzf --ansi \
      --with-nth=1 \
      --delimiter=$'\t' \
      --prompt="❯ " \
      --header='' \
      --border=rounded \
      --border-label=" $ICON tmux palette " \
      --border-label-pos=3 \
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
