local theme = require("config.env")

vim.o.background = "dark"
vim.cmd.colorscheme("oxocarbon")

local function custom_highlights()
  vim.api.nvim_set_hl(0, "FloatBorder", { fg = theme.comment, bg = theme.bg })
  vim.api.nvim_set_hl(0, "MiniStatuslineFilename", { fg = theme.fg, bg = theme.bg })
  vim.api.nvim_set_hl(0, "MiniPickMatchCurrent", { fg = theme.purple })
  -- Match tmux's status bar (bg=default -> theme.bg), so the tabline fill isn't a lighter panel color.
  vim.api.nvim_set_hl(0, "TabLine", { fg = theme.comment, bg = theme.bg })
  vim.api.nvim_set_hl(0, "TabLineFill", { bg = theme.bg })
  vim.api.nvim_set_hl(0, "TabLineSel", { fg = theme.fg, bg = theme.bg })

  -- Mode indicator colors — same accents as the fish vi-mode prompt and tmux status bar.
  vim.api.nvim_set_hl(0, "MiniStatuslineModeNormal", { fg = theme.bg, bg = theme.yellow, bold = true })
  vim.api.nvim_set_hl(0, "MiniStatuslineModeInsert", { fg = theme.bg, bg = theme.green, bold = true })
  vim.api.nvim_set_hl(0, "MiniStatuslineModeVisual", { fg = theme.bg, bg = theme.purple, bold = true })
  vim.api.nvim_set_hl(0, "MiniStatuslineModeReplace", { fg = theme.bg, bg = theme.cyan, bold = true })
  vim.api.nvim_set_hl(0, "MiniStatuslineModeCommand", { fg = theme.bg, bg = theme.orange, bold = true })
  vim.api.nvim_set_hl(0, "MiniStatuslineModeOther", { fg = theme.bg, bg = theme.blue, bold = true })

  -- Git/diagnostics block — its own badge, distinct from the flat filename center.
  vim.api.nvim_set_hl(0, "MiniStatuslineDevinfo", { fg = theme.fg, bg = theme.surface0 })
end

custom_highlights()
vim.api.nvim_create_autocmd("ColorScheme", { callback = custom_highlights })
