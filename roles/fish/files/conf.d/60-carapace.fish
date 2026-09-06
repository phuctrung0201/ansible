# carapace completions — managed by Ansible
# Sourced live at startup so it always matches the installed carapace version.

if type -q carapace
    carapace _carapace fish | source
end
