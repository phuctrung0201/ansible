local function copy_url()
  require("gitlink").copy_url()
end

vim.api.nvim_create_user_command("GitLink", copy_url, { desc = "Copy git remote link for current file" })
vim.keymap.set({ "n", "v" }, "<leader>gy", copy_url, { desc = "Copy Git remote link" })
