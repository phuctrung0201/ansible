# zoxide init — managed by Ansible
# Sourced live at startup so it always matches the installed zoxide version.

if type -q zoxide
    zoxide init fish | source
end
