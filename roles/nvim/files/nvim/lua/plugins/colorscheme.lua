local theme = require("config.env")

local function custom_highlights()
  -- Match tmux's status bar (bg=default -> theme.bg), so the tabline fill isn't a lighter panel color.
  vim.api.nvim_set_hl(0, "TabLine", { fg = theme.comment, bg = theme.bg })
  vim.api.nvim_set_hl(0, "TabLineFill", { bg = theme.bg })
  vim.api.nvim_set_hl(0, "TabLineSel", { fg = theme.fg, bg = theme.bg })

  -- SnacksPickerMatch links to Special by default; use the same accent as fzf's match
  -- highlight (roles/tmux/templates/tmux.conf.j2) for a consistent look.
  vim.api.nvim_set_hl(0, "SnacksPickerMatch", { fg = theme.orange, bold = true })
end

return {
  {
    "olimorris/onedarkpro.nvim",
    lazy = false,
    priority = 1000,
    config = function()
      vim.o.background = "dark"
      vim.cmd.colorscheme("onedark_dark")

      custom_highlights()
      vim.api.nvim_create_autocmd("ColorScheme", { callback = custom_highlights })
    end,
  },

  {
    "LazyVim/LazyVim",
    opts = { colorscheme = "onedark_dark" },
  },
}
