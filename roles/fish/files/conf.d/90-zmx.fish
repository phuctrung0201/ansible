# zmx completions — managed by Ansible
# Sourced live at startup so it always matches the installed zmx version.

if type -q zmx
    zmx completions fish | source
end
