# config.nu — nushell startup configuration
# See https://www.nushell.sh/book/configuration.html

# zoxide must load before other $env.config changes (see zoxide#546)
source ~/.zoxide.nu
source ~/.config/nushell/zoxide-cmd.nu

# env_change.PWD hooks are unreliable with mise; track directories on pre_prompt
$env.__zoxide_last_pwd = ""
$env.config.hooks.pre_prompt = ($env.config.hooks.pre_prompt | append {
  code: {||
    let pwd = $env.PWD
    if $pwd != ($env.__zoxide_last_pwd? | default "") {
      $env.__zoxide_last_pwd = $pwd
      try { ^zoxide add -- $pwd }
    }
  }
})

try { ^zoxide add -- $env.PWD }

$env.config.show_banner = false
$env.config.buffer_editor = "nvim"
$env.config.edit_mode = "vi"

# Hide the date/time right prompt
$env.PROMPT_COMMAND_RIGHT = ""
