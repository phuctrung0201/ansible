# env.nu — loaded before config.nu
# See https://www.nushell.sh/book/configuration.html

$env.XDG_CONFIG_HOME = ($nu.home-dir | path join ".config")

let path_prefixes = [
    ($nu.home-dir | path join ".local" "bin")
    ($nu.current-exe | path dirname)
    "/usr/local/bin"
]
| uniq
| where {|p| ($p | path expand) | path exists }

$env.PATH = ($env.PATH | split row (char esep) | prepend $path_prefixes)

# mise.nu is always generated under XDG config (see ansible nushell role)
use ~/.config/nushell/mise.nu *
