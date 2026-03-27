local function copy_path(expr, label)
  local path = vim.fn.expand(expr)
  vim.fn.setreg("+", path)
  vim.notify("Copied " .. label .. ": " .. path)
end

vim.keymap.set("n", "<leader>br", function()
  copy_path("%:.", "relative path")
end, { desc = "Copy relative path" })
vim.keymap.set("n", "<leader>ba", function()
  copy_path("%:p", "absolute path")
end, { desc = "Copy absolute path" })
vim.keymap.set("n", "<leader>bn", function()
  copy_path("%:t", "filename")
end, { desc = "Copy filename" })

vim.keymap.set("n", "<leader>q", "<cmd>qa!<cr>", { desc = "Force quit" })
vim.keymap.set("n", "<leader>bb", "<cmd>e #<cr>", { desc = "Switch to alternate buffer" })
vim.keymap.set("n", "<Esc>", "<cmd>nohlsearch<cr>", { desc = "Clear search highlights" })
