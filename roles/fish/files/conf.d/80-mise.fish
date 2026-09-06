# mise activation — managed by Ansible
# Sourced live at startup so it always matches the installed mise version.

if type -q mise
    mise activate fish | source
end
