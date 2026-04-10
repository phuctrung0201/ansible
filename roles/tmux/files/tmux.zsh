# Ansible roles/tmux: auto-attach to session "main" (create if missing).
# Disable: export TMUX_AUTOSTART=0 before this file is sourced.

[[ "$TMUX_AUTOSTART" == "0" ]] && return
[[ -o interactive ]] || return
[[ -z "$TMUX" ]] || return
[[ -t 1 ]] || return
command -v tmux &>/dev/null || return

exec tmux new-session -As main
