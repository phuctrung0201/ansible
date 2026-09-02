-- Theme highlights that mirror the tmux status bar.
-- Colors come from lua/config/env.lua (ansible-templated).
local theme = require("config.env")

local M = {}

function M.apply()
  local set = vim.api.nvim_set_hl
  -- Tabline matches tmux's status bar background
  set(0, "TabLine", { fg = theme.comment, bg = theme.bg })
  set(0, "TabLineFill", { bg = theme.bg })
  set(0, "TabLineSel", { fg = theme.fg, bg = theme.bg })
  -- mini.statusline mode colors mirror the tmux mode-color mapping
  set(0, "MiniStatuslineModeNormal", { fg = theme.bg, bg = theme.green })
  set(0, "MiniStatuslineModeInsert", { fg = theme.bg, bg = theme.cyan })
  set(0, "MiniStatuslineModeVisual", { fg = theme.bg, bg = theme.purple })
  set(0, "MiniStatuslineModeReplace", { fg = theme.bg, bg = theme.pink })
  set(0, "MiniStatuslineModeCommand", { fg = theme.bg, bg = theme.orange })
  set(0, "MiniStatuslineModeOther", { fg = theme.bg, bg = theme.comment })
  set(0, "MiniStatuslineDevinfo", { fg = theme.fg, bg = theme.surface0 })
  set(0, "MiniStatuslineFileinfo", { fg = theme.fg, bg = theme.surface0 })
  set(0, "MiniStatuslineFilename", { fg = theme.fg, bg = "NONE" })
  set(0, "MiniStatuslineInactive", { fg = theme.comment, bg = theme.surface0 })
end

return M
