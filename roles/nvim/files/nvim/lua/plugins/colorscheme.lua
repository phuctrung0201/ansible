local theme = require("config.env")

local function custom_highlights()
  -- oxocarbon's NormalFloat bg (#131313) is a different shade than Normal's bg (#161616 ==
  -- theme.bg), so floating windows (Snacks explorer/picker, LSP hover, etc.) look like a
  -- different panel color instead of blending into the editor.
  vim.api.nvim_set_hl(0, "NormalFloat", { fg = theme.fg, bg = theme.bg })
  vim.api.nvim_set_hl(0, "FloatBorder", { fg = theme.comment, bg = theme.bg })
  -- Match tmux's status bar (bg=default -> theme.bg), so the tabline fill isn't a lighter panel color.
  vim.api.nvim_set_hl(0, "TabLine", { fg = theme.comment, bg = theme.bg })
  vim.api.nvim_set_hl(0, "TabLineFill", { bg = theme.bg })
  vim.api.nvim_set_hl(0, "TabLineSel", { fg = theme.fg, bg = theme.bg })

  -- oxocarbon's NonText fg (#393939) is identical to Visual's bg (#393939). Snacks pickers
  -- remap their cursorline to Visual, so any NonText-linked text (dir names, hidden/ignored
  -- files, keymap rhs, ...) becomes invisible on the selected row. Break the collision.
  vim.api.nvim_set_hl(0, "NonText", { fg = theme.comment })

  -- SnacksPickerMatch links to Special, whose fg is identical to Normal's fg in oxocarbon,
  -- so fuzzy-matched characters are indistinguishable from regular text. Same accent as
  -- fzf's match highlight (roles/tmux/templates/tmux.conf.j2) for a consistent look.
  vim.api.nvim_set_hl(0, "SnacksPickerMatch", { fg = theme.orange, bold = true })
end

return {
  {
    "nyoom-engineering/oxocarbon.nvim",
    lazy = false,
    priority = 1000,
    config = function()
      vim.o.background = "dark"
      vim.cmd.colorscheme("oxocarbon")

      custom_highlights()
      vim.api.nvim_create_autocmd("ColorScheme", { callback = custom_highlights })
    end,
  },

  {
    "LazyVim/LazyVim",
    opts = { colorscheme = "oxocarbon" },
  },
}
