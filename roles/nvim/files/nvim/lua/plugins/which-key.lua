return {
  "folke/which-key.nvim",
  event = "VeryLazy",
  opts = {
    delay = 0,
    spec = {
      { "<leader>f", group = "Files" },
      { "<leader>c", group = "Code" },
      { "<leader>d", group = "Database" },
      { "<leader>g", group = "Git" },
      { "<leader>b", group = "Buffers" },
      { "<leader>u", group = "UI" },
      { "<leader>w", group = "Wiki", icon = { icon = "󰎚", color = "purple" } },
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
