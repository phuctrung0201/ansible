vim.api.nvim_create_user_command("GitLink", function()
  require("gitlink").copy_url()
end, { desc = "Copy git remote link for current file" })

vim.keymap.set({ "n", "v" }, "<leader>gy", function()
  require("gitlink").copy_url()
end, { desc = "Copy Git remote link" })
