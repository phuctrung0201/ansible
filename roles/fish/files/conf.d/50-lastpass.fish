# LastPass picker — managed by Ansible.
# Fuzzy-pick an entry with fzf; optional action as $argv[1], else an fzf menu.

function flpass --description 'Fuzzy-pick a LastPass entry and act on it'
    set -q LPASS_AGENT_TIMEOUT; or set -lx LPASS_AGENT_TIMEOUT 0

    set -l icon_key \uf084   # nf-fa-key
    set -l tab (printf '\t')

    set -l action $argv[1]
    if test -z "$action"
        set action (printf 'copy password\ncopy username\ncopy url\ngenerate password\nedit entry\ndelete entry\nadd entry' \
            | fzf --reverse --prompt='LastPass> ')
        or return 0
        switch $action
            case 'copy password'; set action password
            case 'copy username'; set action username
            case 'copy url'; set action url
            case 'generate password'; set action generate
            case 'edit entry'; set action edit
            case 'delete entry'; set action delete
            case 'add entry'; set action add
            case '*'; return 0
        end
    end

    if not command -q lpass
        echo "lpass: lpass-cli not installed"
        return 1
    end

    if not lpass status -q >/dev/null 2>&1
        echo "lpass: not logged in — run: lpass login <email>"
        return 1
    end

    if test "$action" = add
        read -P 'entry name: ' name
        test -z "$name"; and return 0
        lpass add "$name"
        return 0
    end

    # Stream lpass output into fzf so its built-in spinner animates while ls runs.
    set -l selection (lpass ls --format "%aN$tab%ai" 2>/dev/null \
        | fzf --with-nth=1 --delimiter=$tab --prompt="$icon_key lpass ❯ " --no-multi --height=100% --layout=reverse)
    or return 0

    set -l name (printf '%s' "$selection" | cut -f1)
    set -l id (printf '%s' "$selection" | cut -f2)

    switch $action
        case password; lpass show --clip --password "$id"; and echo "lpass: password copied"
        case username; lpass show --clip --username "$id"; and echo "lpass: username copied"
        case url; lpass show --clip --url "$id"; and echo "lpass: url copied"
        case generate; lpass generate --clip "$id" 20; and echo "lpass: password generated"
        case edit; lpass edit "$id"
        case delete
            read -P "delete '$name'? (y/N) " ans
            if string match -qr '^[Yy]$' -- $ans
                lpass rm "$id"; and echo "lpass: deleted"
            end
        case '*'; echo "lpass: unknown action: $action"; return 1
    end
end
