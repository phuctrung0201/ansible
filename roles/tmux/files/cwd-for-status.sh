#!/bin/sh
# Tmux status-left calls this via #(…). No unescaped ')' in the inline #( string — tmux ends #( at the first ).
# Avoid printf formats with % in tmux.conf (status format expands %); use echo here.
p="$1"
h="${HOME%/}"
[ -z "$h" ] && { echo "$p"; exit 0; }
[ "$p" = "$h" ] && { echo '~'; exit 0; }
[ "${p#$h/}" != "$p" ] && { echo "~/${p#$h/}"; exit 0; }
echo "$p"
