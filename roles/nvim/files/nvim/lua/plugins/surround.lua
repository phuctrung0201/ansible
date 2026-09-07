return {
  "kylechui/nvim-surround",
  version = "*",
  event = "VeryLazy",
  init = function()
    vim.g.nvim_surround_no_mappings = true
  end,
  config = function()
    require("nvim-surround").setup()

    vim.keymap.set("n", "gsa", "<Plug>(nvim-surround-normal)", {
      desc = "Add surrounding pair",
    })
    vim.keymap.set("x", "gsa", "<Plug>(nvim-surround-visual)", {
      desc = "Add surrounding pair",
    })
    vim.keymap.set("n", "gsd", "<Plug>(nvim-surround-delete)", {
      desc = "Delete surrounding pair",
    })
    vim.keymap.set("n", "gsr", "<Plug>(nvim-surround-change)", {
      desc = "Replace surrounding pair",
    })
  end,
}
