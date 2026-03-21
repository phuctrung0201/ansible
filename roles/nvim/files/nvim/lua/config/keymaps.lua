-- Keymaps are automatically loaded on the VeryLazy event
-- Default keymaps that are always set: https://github.com/LazyVim/LazyVim/blob/main/lua/lazyvim/config/keymaps.lua
-- Add any additional keymaps here

local function copy_path(expr, label)
  local path = vim.fn.expand(expr)
  vim.fn.setreg("+", path)
  vim.notify("Copied " .. label .. ": " .. path)
end

vim.keymap.set("n", "<leader>byr", function()
  copy_path("%", "relative path")
end, { desc = "Copy relative path" })
vim.keymap.set("n", "<leader>bya", function()
  copy_path("%:p", "absolute path")
end, { desc = "Copy absolute path" })
vim.keymap.set("n", "<leader>byf", function()
  copy_path("%:t", "filename")
end, { desc = "Copy filename" })
