# kitty shell integration (ensures OSC 133 markers for @last_cmd_output etc.)
if [[ -n "$KITTY_INSTALLATION_DIR" ]]; then
  export KITTY_SHELL_INTEGRATION="enabled"
  autoload -Uz -- "$KITTY_INSTALLATION_DIR"/shell-integration/zsh/kitty-integration
  kitty-integration
  unfunction kitty-integration
fi

autoload -Uz edit-command-line
zle -N edit-command-line
bindkey '^x^e' edit-command-line
