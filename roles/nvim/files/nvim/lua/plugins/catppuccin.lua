local theme = require("config.env")

require("catppuccin").setup({
  flavour = "mocha",
  transparent_background = true,
  integrations = { mini = true },
  custom_highlights = function()
    return {
      NormalFloat = { bg = "NONE" },
      FloatBorder = { bg = "NONE" },
      FloatTitle = { bg = "NONE" },
      MsgArea = { bg = "NONE" },
      MiniStatuslineFilename = { fg = theme.fg, bg = theme.surface0 },
      MiniPickMatchCurrent = { fg = theme.pink },
    }
  end,
})

vim.cmd.colorscheme("catppuccin")
