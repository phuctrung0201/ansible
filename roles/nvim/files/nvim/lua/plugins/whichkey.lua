-- Move profiler toggles from <leader>dp to <leader>cp after LazyVim loads.
vim.api.nvim_create_autocmd("User", {
  pattern = "VeryLazy",
  callback = function()
    pcall(vim.keymap.del, "n", "<leader>dpp")
    pcall(vim.keymap.del, "n", "<leader>dph")
    Snacks.toggle.profiler():map("<leader>cpp")
    Snacks.toggle.profiler_highlights():map("<leader>cph")
  end,
})

return {
  {
    "folke/which-key.nvim",
    opts = {
      spec = {
        { "<leader>o", group = "Obsidian", icon = "󱓧" },
        { "<leader>dp", group = "profiler", hidden = true },
        { "<leader>cp", group = "profiler" },
        { "<leader>d", group = "database" },
        { "<leader>d", group = "database", mode = "x" },
      },
    },
  },

  {
    "folke/snacks.nvim",
    keys = {
      { "<leader>dps", false },
      { "<leader>cps", function() Snacks.profiler.scratch() end, desc = "Profiler Scratch Buffer" },
    },
  },
}
