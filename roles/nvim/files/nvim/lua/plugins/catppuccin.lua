local theme = require("config.env")

-- Mirrors the tmux status-left scheme (SHELL=green, PREFIX=orange, COPY=purple)
local mode_colors = {
  Normal = theme.green,
  Insert = theme.blue,
  Visual = theme.purple,
  Replace = theme.red,
  Command = theme.orange,
  Other = theme.yellow,
}

require("catppuccin").setup({
  flavour = "mocha",
  transparent_background = true,
  integrations = { mini = true },
  custom_highlights = function()
    local hl = {
      NormalFloat = { bg = "NONE" },
      FloatBorder = { bg = "NONE" },
      FloatTitle = { bg = "NONE" },
      MiniStatuslineDevinfo = { fg = theme.fg, bg = theme.surface0 },
      MiniStatuslineFilename = { fg = theme.fg, bg = "NONE" },
      MiniStatuslineSepDevToEnd = { fg = theme.surface0, bg = "NONE" },
    }
    for mode, color in pairs(mode_colors) do
      hl["MiniStatuslineMode" .. mode] = { fg = theme.bg, bg = color, style = { "bold" } }
      hl["MiniStatuslineSep" .. mode .. "ToDev"] = { fg = color, bg = theme.surface0 }
      hl["MiniStatuslineSep" .. mode .. "ToEnd"] = { fg = color, bg = "NONE" }
    end
    return hl
  end,
})

vim.cmd.colorscheme("catppuccin")
vim.opt.fillchars = { eob = " " }

local function apply_transparent()
  vim.cmd("highlight MsgArea guibg=NONE ctermbg=NONE")
end
apply_transparent()
vim.api.nvim_create_autocmd("ColorScheme", { callback = apply_transparent })

vim.api.nvim_create_autocmd("VimEnter", {
  once = true,
  callback = function()
    vim.cmd("highlight MiniPickMatchCurrent guifg=" .. theme.pink)
  end,
})
