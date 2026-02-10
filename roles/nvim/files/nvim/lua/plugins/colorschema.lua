return {
  -- add dracula
  {
    "Mofiqul/dracula.nvim",
    opts = {
      transparent_bg = true,
      overrides = function(colors)
        return {
          NormalFloat = { fg = colors.fg, bg = nil },
          SnacksNormal = { fg = colors.fg, bg = nil },
          SnacksNormalNC = { fg = colors.fg, bg = nil },
          SnacksPickerInput = { fg = colors.fg, bg = nil },
          SnacksPickerList = { fg = colors.fg, bg = nil },
          SnacksPickerBox = { fg = colors.fg, bg = nil },
        }
      end,
    },
  },

  -- Configure LazyVim to load dracula
  {
    "LazyVim/LazyVim",
    opts = {
      colorscheme = "dracula",
    },
  },
}
