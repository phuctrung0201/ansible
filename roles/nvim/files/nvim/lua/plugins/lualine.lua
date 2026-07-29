return {
  {
    "nvim-lualine/lualine.nvim",
    opts = function(_, opts)
      opts.sections.lualine_c = { "filename" }
      opts.sections.lualine_y = {}
      opts.sections.lualine_z = {}
      opts.winbar = nil
      opts.inactive_winbar = nil
      return opts
    end,
  },
}
