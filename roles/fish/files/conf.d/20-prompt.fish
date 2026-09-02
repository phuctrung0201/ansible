# Two-line prompt: [mode] cwd + zmx + git, then vi indicator — managed by Ansible.
# Colors come from 10-theme.fish.

# Keep last 4 path segments full (D=2 collapses Documents/integration to D/i).
set -g fish_prompt_pwd_full_dirs 4

# Empty fish_mode_prompt makes fish re-run fish_prompt on vi mode switch
# (otherwise fish_mode_prompt prepends to line 1).
function fish_mode_prompt
end

function __fish_prompt_git --description 'Git branch with icon, no brackets'
    set -l branch (fish_git_prompt '%s')
    test -n "$branch"; or return
    printf ' %s %s' \uf418 $branch
end

function __fish_prompt_mode --description 'vi-mode indicator: a colored icon'
    set -l label
    set -l bg
    # Nerd Font icon + text per mode, mirroring the git segment style.
    set -l icon
    switch $fish_bind_mode
        case insert
            set icon (printf \uf040)  # pencil
            set label insert
            set bg $fish_mode_color_insert
        case default
            set icon (printf \uf111)  # filled circle
            set label normal
            set bg $fish_mode_color_default
        case replace_one replace
            set icon (printf \uf0ec)  # exchange
            set label replace
            set bg $fish_mode_color_replace
        case visual
            set icon (printf \uf06e)  # eye
            set label visual
            set bg $fish_mode_color_visual
        case '*'
            set icon (printf \uf059)  # question circle
            set label (string lower $fish_bind_mode)
            set bg $fish_mode_color_default
    end
    set_color $bg
    printf '%s %s' $icon $label
    set_color normal
end

function fish_prompt --description 'Two-line prompt: [mode] cwd + zmx + git, then arrow'
    set_color $fish_color_cwd
    echo -n (prompt_pwd)
    echo -n ' '(__fish_prompt_mode)
    if set -q ZMX_SESSION
        set_color $fish_color_zmx
        printf ' %s %s' \uf1e6 $ZMX_SESSION
        set_color normal
    end
    set_color $fish_color_vcs
    echo -n (__fish_prompt_git)
    set_color normal
    echo

    set_color --bold $fish_color_cwd
    echo -n '❯ '
    set_color normal
end
