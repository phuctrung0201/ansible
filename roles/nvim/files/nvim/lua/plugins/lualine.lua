-- Mirrors the tmux status bar's mode-color mapping (roles/tmux/templates/tmux.conf.j2):
-- idle/normal = green, prefix/command = orange, copy/visual = purple.
local theme = require("config.env")

local function mode_hl(color)
  return { a = { bg = color, fg = theme.bg, gui = "bold" }, b = { bg = theme.surface0, fg = theme.fg } }
end

local lualine_theme = {
  normal = vim.tbl_extend("force", mode_hl(theme.green), { c = { bg = "none", fg = theme.fg } }),
  insert = mode_hl(theme.cyan),
  visual = mode_hl(theme.purple),
  replace = mode_hl(theme.pink),
  command = mode_hl(theme.orange),
  inactive = {
    a = { bg = theme.surface0, fg = theme.comment },
    b = { bg = theme.surface0, fg = theme.comment },
    c = { bg = "none", fg = theme.comment },
  },
}

return {
  {
    "nvim-lualine/lualine.nvim",
    opts = function(_, opts)
      opts.options.theme = lualine_theme
      opts.sections.lualine_c = { "filename" }
      opts.sections.lualine_y = {}
      opts.sections.lualine_z = {}
      opts.winbar = nil
      opts.inactive_winbar = nil
      return opts
    end,
  },
}
