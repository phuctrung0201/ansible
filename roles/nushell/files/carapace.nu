# Carapace external command completions (docker, kubectl, git, etc.)
# https://www.nushell.sh/cookbook/external_completers.html#carapace-completer

$env.CARAPACE_LENIENT = 1

$env.config.completions.external = {
    enable: true
    completer: {|spans|
        carapace $spans.0 nushell ...$spans | from json
    }
}
