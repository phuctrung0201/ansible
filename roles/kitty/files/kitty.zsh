autoload -Uz edit-command-line
zle -N edit-command-line
bindkey '^x^e' edit-command-line

kitty-tab-switch() {
  local selected
  selected=$(kitten @ ls | jq -r '
    [.[] | select(any(.tabs[].windows[]; .is_self))
      | .tabs[] | select(any(.windows[]; .is_self) | not)
      | {id: .id, title: .title}]
    | sort_by(.id)
    | .[]
    | "\(.id)\t\(.title)"
  ' | fzf --prompt="📑 Switch Tab > " \
          --layout=reverse \
          --height=100% \
          --no-info \
          --delimiter=$'\t' \
          --with-nth=2 \
          --ansi | cut -f1)

  [[ -z "$selected" ]] && return 0
  kitten @ close-window --self
  kitten @ focus-tab --match id:"$selected"
}
