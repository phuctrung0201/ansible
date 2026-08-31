# Two-line prompt: cwd + git, then vi indicator — managed by Ansible.
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
    printf '%s %s %s' ' ' \uf418 $branch
end

function fish_prompt --description 'Two-line prompt: zmx + cwd + git, then vi indicator'
    set_color $fish_color_cwd
    echo -n (prompt_pwd)
    if set -q ZMX_SESSION
        set_color $fish_color_zmx
        printf ' %s %s' \uf1e6 $ZMX_SESSION
        set_color normal
    end
    set_color $fish_color_vcs
    echo -n (__fish_prompt_git)
    set_color normal
    echo

    switch $fish_bind_mode
        case insert
            set_color --bold $fish_mode_color_insert
            echo -n '❯ '
        case default
            set_color --bold $fish_mode_color_default
            echo -n '❮ '
        case replace_one replace
            set_color --bold $fish_mode_color_replace
            echo -n '▼ '
        case visual
            set_color --bold $fish_mode_color_visual
            echo -n '◆ '
    end
    set_color normal
end
