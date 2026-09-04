return {
  "catppuccin/nvim",
  name = "catppuccin",
  lazy = false,
  priority = 1000,
  config = function()
    require("catppuccin").setup({
      flavour = "mocha",
      -- Darker background than stock Mocha: drop base to Crust and shift the
      -- mantle/crust layers darker so surfaces still read distinct from the editor.
      color_overrides = {
        mocha = {
          base = "#11111b",
          mantle = "#0d0d15",
          crust = "#0b0b13",
        },
      },
    })
    vim.o.background = "dark"
    vim.cmd.colorscheme("catppuccin-mocha")
  end,
}
