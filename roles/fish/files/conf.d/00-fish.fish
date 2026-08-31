# fish core: shell settings, PATH, vi mode — managed by Ansible

set -g fish_greeting

set -gx EDITOR nvim
set -gx PAGER bat
set -gx CARAPACE_BRIDGES "zsh,fish,bash,inshellisense"

fish_add_path -gp "$HOME/.local/bin" "$HOME/.config/carapace/bin" (brew --prefix)/bin

# Custom binds live in fish_user_key_bindings (see 30-clipboard.fish),
# which fish invokes automatically after loading vi bindings.
set -g fish_key_bindings fish_vi_key_bindings
