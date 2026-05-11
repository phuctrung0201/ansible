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
Session: rename to current folder	tmux	!tmux rename-session "$(basename "$(tmux display-message -p '#{pane_current_path}')")"
Session: detach	tmux	detach-client
Session: kill all others	tmux	kill-session -a
Window: rename to agent	tmux	rename-window agent
Window: rename to markserv	tmux	rename-window markserv
Window: kill current	tmux	kill-window
Window: kill all others	tmux	kill-window -a
Window: new	tmux	new-window
Window: move to another session	tmux	!move_window_to_session
Pane: kill current	tmux	kill-pane
Pane: kill all others	tmux	kill-pane -a
Pane: break out to new window	tmux	break-pane
Pane: split left	tmux	split-window -hb -c "#{pane_current_path}"
Pane: split right	tmux	split-window -h -c "#{pane_current_path}"
Pane: split up	tmux	split-window -vb -c "#{pane_current_path}"
Pane: split down	tmux	split-window -v -c "#{pane_current_path}"
Pane: swap left	tmux	swap-pane -s '{left-of}'
Pane: swap down	tmux	swap-pane -s '{down-of}'
Pane: swap up	tmux	swap-pane -s '{up-of}'
Pane: swap right	tmux	swap-pane -s '{right-of}'
Pane: move to another window	tmux	!move_pane_to_window
Credential: copy password	lpass	!~/.config/tmux/lpass.sh password
Credential: copy username	lpass	!~/.config/tmux/lpass.sh username
Credential: add new	lpass	!~/.config/tmux/lpass.sh add
Credential: generate password	lpass	!~/.config/tmux/lpass.sh generate
EOF
)

# Dynamic: one entry per existing session and per window in current session.
sessions=$(tmux list-sessions -F '#{session_name}' 2>/dev/null \
  | awk -v OFS='\t' '{print "Session: switch to " $0, "tmux", "switch-client -t \"" $0 "\""}')

windows=$(tmux list-windows -F $'#{window_index}\t#{window_name}' 2>/dev/null \
  | awk -F'\t' -v OFS='\t' '{print "Window: switch to " $2, "tmux", "select-window -t " $1}')

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
