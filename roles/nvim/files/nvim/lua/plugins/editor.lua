return {
  { "terrastruct/d2-vim", ft = "d2" },

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
