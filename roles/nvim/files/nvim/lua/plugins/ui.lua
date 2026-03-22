return {
  {
    "folke/which-key.nvim",
    event = "VeryLazy",
    opts = {
      spec = {
        { "<leader>b", group = "Buffer" },
        { "<leader>by", group = "Yank path" },
        { "<leader>f", group = "Find" },
        { "<leader>g", group = "Git" },
      },
    },
  },

  {
    "folke/snacks.nvim",
    lazy = false,
    priority = 1000,
    keys = {
      { "<leader>e", function() Snacks.explorer() end, desc = "Toggle file explorer" },
    },
    opts = {
      dashboard = { enabled = true },
      explorer = { enabled = true },
      notifier = { enabled = true },
      indent = { enabled = true },
      words = { enabled = true },
      statuscolumn = { enabled = true },
    },
  },
}
