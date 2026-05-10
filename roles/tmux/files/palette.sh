#!/usr/bin/env bash
# tmux command palette — fuzzy-pick a useful tmux command
# Invoked from a `display-popup -E` binding; runs the chosen command via tmux.

set -euo pipefail

# Nerd-font glyph (nf-fa-terminal) — printf'd to keep the source pure ASCII
ICON=$(printf '\xef\x84\xa0')

# Catppuccin Mocha overlay0 — comment-like dim grey for the hint column.
HINT=$'\e[38;2;108;112;134m'
RESET=$'\e[0m'

FZF_OPTS=(--height=100% --layout=reverse --border=rounded)

# Helpers used by !-prefixed entries that need a secondary picker.
move_window_to_session() {
  local cur t
  cur=$(tmux display-message -p '#{session_name}')
  t=$(tmux list-sessions -F '#{session_name}' \
      | awk -v c="$cur" '$0 != c' \
      | fzf "${FZF_OPTS[@]}" --prompt="to session ❯ ")
  [ -z "$t" ] && return 0
  tmux move-window -t "$t"
}

move_pane_to_window() {
  local cur t
  cur=$(tmux display-message -p '#{window_index}')
  t=$(tmux list-windows -F '#{window_index} (#{window_name})' \
      | awk -v c="$cur" '$1 != c' \
      | fzf "${FZF_OPTS[@]}" --prompt="to window ❯ " \
      | awk '{print $1}')
  [ -z "$t" ] && return 0
  tmux join-pane -t ":$t"
}

# Format per line: <label>\t<hint>\t<command>
# Default: <command> is a tmux subcommand (we run `tmux <cmd>`).
# Lines starting with `!` run as raw shell — these can call helpers above,
# external scripts, or use $(...) substitution.
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
Move window to session	tmux	!move_window_to_session
Move pane to window	tmux	!move_pane_to_window
Swap pane left	tmux	swap-pane -s '{left-of}'
Swap pane down	tmux	swap-pane -s '{down-of}'
Swap pane up	tmux	swap-pane -s '{up-of}'
Swap pane right	tmux	swap-pane -s '{right-of}'
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

# Render <label>  <colored-hint>\t<cmd> so fzf shows the hint as a tight
# inline suffix instead of a tab-stop-wide gap. `sort -f` alphabetizes.
selection=$(printf '%s\n%s\n%s\n' "$static" "$sessions" "$windows" \
  | awk -F'\t' -v D="$HINT" -v R="$RESET" 'NF { printf "%s  %s%s%s\t%s\n", $1, D, $2, R, $3 }' \
  | sort -f \
  | fzf "${FZF_OPTS[@]}" --ansi --with-nth=1 --delimiter=$'\t' --no-multi \
      --prompt="❯ " --border-label=" $ICON tmux palette " --border-label-pos=3) || exit 0

cmd=$(printf '%s' "$selection" | cut -f2-)
[ -z "$cmd" ] && exit 0

if [[ "$cmd" == !* ]]; then
  eval "${cmd#!}"
else
  eval "tmux $cmd"
fi
