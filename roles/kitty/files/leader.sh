#!/usr/bin/env zsh
set -eo pipefail

export PATH="/opt/homebrew/bin:$PATH"

typeset -A cmds descriptions

cmds=(
  c 'kitten hints --program @'
  o 'open_url_with_hints'
  e '\x18\x05'
  r '\x12'
  l 'copy_last_command_output'
  s 'show_scrollback'
  w 'launch --type=os-window --cwd=current'
)

descriptions=(
  c "Copy URL to clipboard"
  o "Open URL in browser"
  e "Edit command in $EDITOR"
  r "Reverse search history"
  l "Copy last command output"
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
             --ansi | awk '{print $1}'
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
