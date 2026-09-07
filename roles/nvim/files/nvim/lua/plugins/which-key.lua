return {
  "folke/which-key.nvim",
  event = "VeryLazy",
  opts = {
    delay = 0,
    spec = {
      { "<leader>s", group = "Search" },
      { "<leader>d", group = "Database" },
      { "<leader>g", group = "Git" },
      { "<leader>b", group = "Buffer" },
      { "<leader>t", group = "Toggle" },
      { "<leader>o", group = "Obsidian", icon = { icon = "󰎚", color = "purple" } },
    },
  },
  keys = {
    {
      "<leader>?",
      function()
        require("which-key").show({ global = false })
      end,
      desc = "Buffer-local keymaps",
    },
  },
}
