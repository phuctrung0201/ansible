# Vi yanks/pastes use the system clipboard (pbcopy/pbpaste on macOS).
# All custom vi binds live here; fish calls fish_user_key_bindings
# automatically after loading vi bindings.

function __fish_yank_to_clipboard --description 'Copy latest yank to system clipboard'
    test -n "$fish_killring[1]"; or return
    printf '%s' "$fish_killring[1]" | fish_clipboard_copy
end

function fish_vi_yank_selection --description 'Yank selection and sync to system clipboard'
    set -g fish_cursor_end_mode exclusive
    commandline -f kill-selection -f yank
    set -g fish_cursor_end_mode inclusive
    __fish_yank_to_clipboard
end

function fish_user_key_bindings --description 'Clipboard vi yank/paste + word-by-word history'
    bind -M default p 'set -g fish_cursor_end_mode exclusive; commandline -f forward-char; set -g fish_cursor_end_mode inclusive; fish_clipboard_paste'
    bind -M default P fish_clipboard_paste

    bind -M operator y 'fish_vi_exec_motion --linewise; __fish_yank_to_clipboard'

    bind -M default Y 'commandline -f kill-whole-line yank; __fish_yank_to_clipboard'
    bind -M default y,\$ 'commandline -f kill-line yank; __fish_yank_to_clipboard'
    bind -M default y,\^ 'commandline -f backward-kill-line yank; __fish_yank_to_clipboard'
    bind -M default y,0 'commandline -f backward-kill-line yank; __fish_yank_to_clipboard'
    bind -M default y,i,w 'commandline -f kill-inner-word yank; __fish_yank_to_clipboard'
    bind -M default y,i,W 'commandline -f kill-inner-bigword yank; __fish_yank_to_clipboard'
    bind -M default y,a,w 'commandline -f kill-a-word yank; __fish_yank_to_clipboard'
    bind -M default y,a,W 'commandline -f kill-a-bigword yank; __fish_yank_to_clipboard'

    # Word-by-word history: Alt+l forward, Alt+h backward (macOS blocks Ctrl+Arrow).
    bind -M insert \el history-search-forward
    bind -M default \el history-search-forward
    bind -M insert \eh history-search-backward
    bind -M default \eh history-search-backward

    # Edit current command line in $EDITOR: Opt+Return (\e\r; \e\n fallback).
    bind -M insert \e\r edit_command_buffer
    bind -M default \e\r edit_command_buffer
    bind -M insert \e\n edit_command_buffer
    bind -M default \e\n edit_command_buffer
end
