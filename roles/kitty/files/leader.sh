#!/usr/bin/env zsh
set -eo pipefail

export PATH="/opt/homebrew/bin:$PATH"

typeset -A cmds descriptions

cmds=(
  u  'open_url_with_hints'
  uy 'kitten hints --program @'
  p  'kitten hints --type=path --program=@'
  e  '\x18\x05'
  h '\x12'
  l 'copy_last_command_output'
  s 'show_scrollback'
  w 'launch --type=os-window --cwd=current'
)

descriptions=(
  u  "Hint URL to open"
  uy "Hint URL to yank"
  p  "Hint path to yank"
  e  "Edit command in $EDITOR"
  h "Reverse search history"
  l "Yank last command output"
  s "Browse scrollback buffer"
  w "Open window in current dir"
)

selection=$(
  for key in ${(ko)descriptions}; do
    printf "\033[33m%-4s\033[0m %s\n" "$key" "${descriptions[$key]}"
  done | fzf --prompt="⌨ Which Keys > " \
             --layout=reverse \
             --height=100% \
             --no-info \
             --ansi \
             --tiebreak=begin | awk '{print $1}'
)

[[ -z "$selection" ]] && exit 0

cmd="${cmds[$selection]}"

trap '' HUP
kitten @ close-window --self

if [[ "$cmd" == \\x* ]]; then
  kitten @ send-text "$cmd"
else
  kitten @ action "$cmd"
fi
