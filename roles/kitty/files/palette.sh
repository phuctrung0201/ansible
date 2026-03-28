#!/usr/bin/env zsh
set -eo pipefail

export PATH="/opt/homebrew/bin:$PATH"

typeset -A actions cmds descriptions

actions=(
  copy    action
  edit    send-text
  fix       send-text
  history   send-text
  last      action
  scroll    action
  open      action
)

cmds=(
  copy    'kitten hints --program @'
  edit    '\x18\x05'
  fix       'fc\n'
  history   '\x12'
  last      show_last_command_output
  scroll    show_scrollback
  open      open_url_with_hints
)

descriptions=(
  copy    "Copy URL"
  edit    "Edit command line"
  fix       "Fix last command"
  history   "Search history"
  last      "Last output"
  scroll    "Scrollback"
  open      "Open URL"
)

selection=$(
  for key in ${(ko)descriptions}; do
    printf "%-22s  %s\n" "$key" "${descriptions[$key]}"
  done | fzf --prompt=" " \
             --header="⌨ Palette" \
             --layout=reverse \
             --height=100% \
             --no-info | awk '{print $1}'
)

[[ -z "$selection" ]] && exit 0

type="${actions[$selection]}"
cmd="${cmds[$selection]}"

trap '' HUP
kitten @ close-window --self

case "$type" in
  send-text) kitten @ send-text "$cmd" ;;
  action)    kitten @ action "$cmd" ;;
esac
