return {
  {
    "neovim/nvim-lspconfig",
    opts = {
      servers = {
        sqls = { enabled = false },
      },
    },
  },

  {
    "kristijanhusak/vim-dadbod-ui",
    keys = {
      { "<leader>D", "<cmd>DBUIToggle<cr>", desc = "Toggle DBUI" },
    },
  },
}
