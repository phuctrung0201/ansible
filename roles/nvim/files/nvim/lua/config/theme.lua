-- Theme highlights for the tabline and floating windows.
-- Colors come from lua/config/env.lua (ansible-templated Catppuccin Mocha palette).
-- The editor background is Crust (see colorscheme.lua color_overrides), so the
-- tabline background uses `crust` to match.
local theme = require("config.env")

local M = {}

function M.apply()
  local set = vim.api.nvim_set_hl
  -- Tabline matches the editor background (Crust)
  set(0, "TabLine", { fg = theme.overlay0, bg = theme.crust })
  set(0, "TabLineFill", { bg = theme.crust })
  set(0, "TabLineSel", { fg = theme.text, bg = theme.crust })
  -- Floating windows: match the editor background (Crust) so there's no
  -- contrasting/black frame around floats. Border glyphs keep the colorscheme's
  -- color (only their background is set). nvim-tree's float window links to these.
  set(0, "NormalFloat", { fg = theme.text, bg = theme.crust })
  for _, name in ipairs({ "FloatBorder", "FloatTitle" }) do
    local hl = vim.api.nvim_get_hl(0, { name = name, link = false })
    hl.bg = theme.crust
    set(0, name, hl)
  end
  -- Keep nvim-tree's body, gutter, and surrounding chrome on the same
  -- background. Catppuccin otherwise assigns its darker mantle/crust layers,
  -- which show up as rectangular blocks in the floating explorer.
  for _, name in ipairs({
    "NvimTreeNormal",
    "NvimTreeNormalNC",
    "NvimTreeNormalFloat",
    "NvimTreeNormalFloatBorder",
    "NvimTreeEndOfBuffer",
    "NvimTreeSignColumn",
    "NvimTreeLineNr",
    "NvimTreeStatusLine",
    "NvimTreeStatusLineNC",
    "NvimTreeWinSeparator",
  }) do
    local hl = vim.api.nvim_get_hl(0, { name = name, link = false })
    hl.bg = theme.crust
    set(0, name, hl)
  end
  -- Match the explorer's rounded border to the blue picker border.
  set(0, "NvimTreeNormalFloatBorder", { fg = theme.blue, bg = theme.crust })
end

return M
