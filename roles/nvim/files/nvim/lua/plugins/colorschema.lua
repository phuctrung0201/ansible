return {
  {
    "Mofiqul/dracula.nvim",
    lazy = false,
    priority = 1000,
    opts = {
      transparent_bg = true,
      overrides = function(colors)
        return {
          NormalFloat = { fg = colors.fg, bg = nil },
        }
      end,
    },
    config = function(_, opts)
      require("dracula").setup(opts)
      vim.cmd.colorscheme("dracula")
    end,
  },
}
