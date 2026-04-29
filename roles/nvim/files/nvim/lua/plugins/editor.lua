return {
  { "terrastruct/d2-vim", ft = "d2" },

  {
    "folke/snacks.nvim",
    opts = {
      picker = {
        win = {
          input = {
            keys = {
              ["<c-g>"] = { "toggle_ignored", mode = { "i", "n" } },
              ["<c-h>"] = { "toggle_hidden", mode = { "i", "n" } },
            },
          },
          list = {
            keys = {
              ["<c-g>"] = "toggle_ignored",
              ["<c-h>"] = "toggle_hidden",
            },
          },
        },
      },
    },
  },


  {
    "nvim-treesitter/nvim-treesitter",
    opts = {
      ensure_installed = { "c_sharp" },
    },
  },

  {
    "akinsho/bufferline.nvim",
    opts = function(_, opts)
      opts.options = opts.options or {}
      opts.options.offsets = vim.tbl_filter(function(o)
        return o.filetype ~= "snacks_layout_box"
      end, opts.options.offsets or {})
      return opts
    end,
  },

}
