#!/usr/bin/env bash
# LastPass picker — fuzzy-pick an entry; action is passed as the first argument.
# Used by the tmux command palette and callable from zsh: ~/.config/zsh/lpass.sh <action>

set -euo pipefail

action=${1:-password}

# Nerd-font glyph (printf'd to keep the source pure ASCII)
ICON_KEY=$(printf '\xef\x82\x84')   # nf-fa-key

notify() {
  if [[ -n "${TMUX:-}" ]]; then
    tmux display-message -d 2000 "lpass: $*"
  else
    echo "lpass: $*"
  fi
}

if ! command -v lpass >/dev/null 2>&1; then
  notify "lpass-cli not installed"
  exit 1
fi

if ! lpass status -q >/dev/null 2>&1; then
  notify "not logged in — run: lpass login <email>"
  exit 1
fi

if [ "$action" = "add" ]; then
  read -r -p 'entry name: ' name
  [ -z "$name" ] && exit 0
  lpass add "$name"
  exit 0
fi

TAB=$'\t'

# Stream lpass output into fzf so its built-in spinner animates while ls runs.
selection=$(lpass ls --format "%aN${TAB}%ai" 2>/dev/null | \
  fzf --with-nth=1 \
      --delimiter="$TAB" \
      --prompt="$ICON_KEY lpass ❯ " \
      --no-multi \
      --height=100% \
      --layout=reverse) || exit 0

name=$(printf '%s' "$selection" | cut -f1)
id=$(printf '%s' "$selection" | cut -f2)

case "$action" in
  password) lpass show --clip --password "$id" && notify "password copied" ;;
  username) lpass show --clip --username "$id" && notify "username copied" ;;
  url)      lpass show --clip --url "$id"      && notify "url copied" ;;
  generate) lpass generate --clip "$id" 20     && notify "password generated" ;;
  edit)     lpass edit "$id" ;;
  delete)
    read -r -p "delete '$name'? (y/N) " ans
    [[ "$ans" =~ ^[Yy]$ ]] && lpass rm "$id" && notify "deleted"
    ;;
  *) notify "unknown action: $action"; exit 1 ;;
esac
