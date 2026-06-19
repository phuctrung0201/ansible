# z/zi wrappers with tab completion (required since nushell 0.103)
# https://www.nushell.sh/cookbook/custom_completers.html

def "nu-complete zoxide path" [context: string] {
  let parts = ($context | split row " " | skip 1)
  {
    options: {
      sort: false
      completion_algorithm: "substring"
      case_sensitive: false
    }
    completions: (^zoxide query --list --exclude $env.PWD -- ...$parts | lines)
  }
}

def --env --wrapped z [...rest: string@"nu-complete zoxide path"] {
  __zoxide_z ...$rest
}

def --env --wrapped zi [...rest: string] {
  __zoxide_zi ...$rest
}
