#!/usr/bin/env zsh
set -eo pipefail

export PATH="/opt/homebrew/bin:$PATH"

actions=(
  $'Hints: Open URL\topen_url_with_hints'
  $'Hints: Copy URL\tkitten hints --program @'
  $'Hints: Copy Path\tkitten hints --type=path --program=@'
  $'Terminal: Edit Command Line\t\\x18\\x05'
  $'Terminal: Search History\t\\x12'
  $'Terminal: Copy Last Output\tcopy_last_command_output'
  $'Terminal: Open Scrollback\tshow_scrollback'
  $'Window: New Window\tlaunch --type=os-window'
  $'Window: New Window (cwd)\tlaunch --type=os-window --cwd=current'
  $'Tab: New Tab\tlaunch --type=tab'
  $'Tab: New Tab (cwd)\tlaunch --type=tab --cwd=current'
  $'Tab: Switch Tab\t__tab_switch__'
  $'Tab: Close Other Tabs\t__close_other_tabs__'
  $'Tab: Move to New Window\tdetach_tab'
)

selection=$(
  printf '%s\n' "${actions[@]}" \
  | fzf --prompt="⚡ palette > " \
        --layout=reverse \
        --height=100% \
        --no-info \
        --delimiter=$'\t' \
        --nth=1 \
        --with-nth=1 \
        --tiebreak=index
)

[[ -z "$selection" ]] && exit 0

cmd="${selection#*$'\t'}"

if [[ "$cmd" == '__close_other_tabs__' ]]; then
  tab_ids=$(kitten @ ls | jq -r '
    [.[] | select(any(.tabs[].windows[]; .is_self))] | .[0]
    | .tabs[] | select(any(.windows[]; .is_self) | not)
    | .id
  ')
  trap '' HUP
  kitten @ close-window --self
  for tid in ${(f)tab_ids}; do
    kitten @ close-tab --match id:"$tid"
  done
  exit 0
fi

if [[ "$cmd" == '__tab_switch__' ]]; then
  tab_id=$(kitten @ ls | jq -r '
    [.[] | select(any(.tabs[].windows[]; .is_self))
      | .tabs[] | select(any(.windows[]; .is_self) | not)
      | {id: .id, title: .title, wid: .active_window_history[0]}]
    | sort_by(.id)
    | .[]
    | "\(.id)\t\(.wid)\t\(.title)"
  ' | fzf --prompt="📑 tab > " \
        --layout=reverse \
        --height=100% \
        --no-info \
        --delimiter=$'\t' \
        --with-nth=3 \
        --ansi \
        --preview 'kitten @ get-text --ansi --match id:{2}' \
  | cut -f1)
  [[ -z "$tab_id" ]] && exit 0
  trap '' HUP
  kitten @ close-window --self
  kitten @ focus-tab --match id:"$tab_id"
  exit 0
fi

trap '' HUP
kitten @ close-window --self

if [[ "$cmd" == \\x* ]]; then
  kitten @ send-text "$cmd"
else
  kitten @ action "$cmd"
fi
