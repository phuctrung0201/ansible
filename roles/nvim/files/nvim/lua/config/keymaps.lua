vim.keymap.set("t", "<A-\\>", "<C-\\><C-n>", { desc = "Exit terminal mode" })

-- Move the profiler toggles from <leader>dp (debug) to <leader>cp (code).
-- LazyVim binds these via Snacks.toggle on VeryLazy, so re-map after it runs.
vim.api.nvim_create_autocmd("User", {
  pattern = "VeryLazy",
  callback = function()
    pcall(vim.keymap.del, "n", "<leader>dpp")
    pcall(vim.keymap.del, "n", "<leader>dph")
    Snacks.toggle.profiler():map("<leader>cpp")
    Snacks.toggle.profiler_highlights():map("<leader>cph")
  end,
})
