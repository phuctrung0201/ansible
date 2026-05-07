#!/usr/bin/env bash
# tmux LastPass picker — fuzzy-pick an entry, then an action.
# Invoked from the tmux command palette inside a `display-popup -E`.

set -euo pipefail

# Nerd-font glyphs (printf'd to keep the source pure ASCII)
ICON_KEY=$(printf '\xef\x82\x84')   # nf-fa-key
ICON_BOLT=$(printf '\xef\x83\xa7')  # nf-fa-bolt

notify() { tmux display-message -d 2000 "lpass: $*"; }

if ! command -v lpass >/dev/null 2>&1; then
  notify "lpass-cli not installed"
  exit 1
fi

if ! lpass status -q >/dev/null 2>&1; then
  notify "not logged in — run: lpass login <email>"
  exit 1
fi

TAB=$'\t'

# Stream lpass output into fzf so its built-in spinner animates while ls runs.
selection=$({ printf '+ add new entry\n'; lpass ls --format "%aN${TAB}%ai" 2>/dev/null; } | \
  fzf --with-nth=1 \
      --delimiter="$TAB" \
      --prompt="$ICON_KEY lpass ❯ " \
      --no-multi \
      --height=100% \
      --layout=reverse) || exit 0

if [ "$selection" = "+ add new entry" ]; then
  read -r -p 'entry name: ' name
  [ -z "$name" ] && exit 0
  lpass add "$name"
  exit 0
fi

name=$(printf '%s' "$selection" | cut -f1)
id=$(printf '%s' "$selection" | cut -f2)

action=$(printf '%s\n' \
  'copy password' \
  'copy username' \
  'copy url' \
  'generate password' \
  'edit' \
  'delete' \
  | fzf --prompt="$ICON_BOLT $name ❯ " --no-multi --height=100% --layout=reverse) || exit 0

case "$action" in
  'copy password')    lpass show --clip --password "$id" && notify "password copied" ;;
  'copy username')    lpass show --clip --username "$id" && notify "username copied" ;;
  'copy url')         lpass show --clip --url "$id"      && notify "url copied" ;;
  'generate password') lpass generate --clip "$id" 20    && notify "password generated" ;;
  edit)               lpass edit "$id" ;;
  delete)
    read -r -p "delete '$name'? (y/N) " ans
    [[ "$ans" =~ ^[Yy]$ ]] && lpass rm "$id" && notify "deleted"
    ;;
esac
