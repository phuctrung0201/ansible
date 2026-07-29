local theme = require("config.env")

return {
  {
    "catppuccin/nvim",
    name = "catppuccin",
    lazy = false,
    priority = 1000,
    opts = {
      flavour = "mocha",
      transparent_background = true,
      integrations = { snacks = true },
      custom_highlights = function()
        return {
          NormalFloat = { bg = "NONE" },
          FloatBorder = { bg = "NONE" },
          FloatTitle = { bg = "NONE" },
        }
      end,
    },
    config = function(_, opts)
      require("catppuccin").setup(opts)
      vim.cmd.colorscheme("catppuccin")
      vim.opt.fillchars = { eob = " " }

      local function apply_transparent()
        vim.cmd("highlight MsgArea guibg=NONE ctermbg=NONE")
      end
      apply_transparent()
      vim.api.nvim_create_autocmd("ColorScheme", { callback = apply_transparent })

      vim.api.nvim_create_autocmd("User", {
        pattern = "VeryLazy",
        once = true,
        callback = function()
          local cl = vim.api.nvim_get_hl(0, { name = "CursorLine", link = false })
          vim.api.nvim_set_hl(0, "SnacksCursorLine", cl)
          vim.cmd("highlight SnacksPickerDir   guifg=" .. theme.comment)
          vim.cmd("highlight SnacksPickerMatch guifg=" .. theme.pink)
        end,
      })
    end,
  },
}
