-- Theme highlights for the tabline + mini.statusline.
-- Colors come from lua/config/env.lua (ansible-templated Catppuccin Mocha palette).
-- The editor background is Crust (see colorscheme.lua color_overrides), so the
-- tabline/statusline backgrounds use `crust` to match, and mode blocks use `crust`
-- as their (dark) foreground against the colored accent backgrounds.
local theme = require("config.env")

local M = {}

function M.apply()
  local set = vim.api.nvim_set_hl
  -- Tabline matches the editor background (Crust)
  set(0, "TabLine", { fg = theme.overlay0, bg = theme.crust })
  set(0, "TabLineFill", { bg = theme.crust })
  set(0, "TabLineSel", { fg = theme.text, bg = theme.crust })
  -- mini.statusline mode colors (Catppuccin accents; dark Crust text on top)
  set(0, "MiniStatuslineModeNormal", { fg = theme.crust, bg = theme.green })
  set(0, "MiniStatuslineModeInsert", { fg = theme.crust, bg = theme.teal })
  set(0, "MiniStatuslineModeVisual", { fg = theme.crust, bg = theme.mauve })
  set(0, "MiniStatuslineModeReplace", { fg = theme.crust, bg = theme.pink })
  set(0, "MiniStatuslineModeCommand", { fg = theme.crust, bg = theme.peach })
  set(0, "MiniStatuslineModeOther", { fg = theme.crust, bg = theme.overlay1 })
  set(0, "MiniStatuslineDevinfo", { fg = theme.text, bg = theme.surface0 })
  set(0, "MiniStatuslineFileinfo", { fg = theme.text, bg = theme.surface0 })
  set(0, "MiniStatuslineFilename", { fg = theme.text, bg = "NONE" })
  set(0, "MiniStatuslineInactive", { fg = theme.overlay0, bg = theme.surface0 })
  -- Floating windows: match the editor background (Crust) so there's no
  -- contrasting/black frame around floats. Border glyphs keep the colorscheme's
  -- color (only their background is set). neo-tree's float groups link to these.
  set(0, "NormalFloat", { fg = theme.text, bg = theme.crust })
  for _, name in ipairs({ "FloatBorder", "FloatTitle" }) do
    local hl = vim.api.nvim_get_hl(0, { name = name, link = false })
    hl.bg = theme.crust
    set(0, name, hl)
  end
end

return M
