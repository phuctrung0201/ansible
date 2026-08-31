# eza aliases — managed by Ansible

if type -q eza
    alias ls='eza --icons'
    alias ll='eza -la --icons'
    alias la='eza -a --icons'
    alias lt='eza --tree --level=2 --icons'
end
