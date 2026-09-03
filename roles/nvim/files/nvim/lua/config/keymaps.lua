local map = vim.keymap.set

-- General
map("n", "<Esc>", "<cmd>nohlsearch<cr>")
map("t", "<Esc><Esc>", "<C-\\><C-n>", { desc = "Exit terminal mode" })

-- Window navigation
map("n", "<C-h>", "<C-w><C-h>", { desc = "Move focus to the left window" })
map("n", "<C-l>", "<C-w><C-l>", { desc = "Move focus to the right window" })
map("n", "<C-j>", "<C-w><C-j>", { desc = "Move focus to the lower window" })
map("n", "<C-k>", "<C-w><C-k>", { desc = "Move focus to the upper window" })

-- Diagnostics
map("n", "<leader>q", vim.diagnostic.setloclist, { desc = "Open diagnostic quickfix list" })

-- Toggle
map("n", "<leader>tw", function()
  vim.wo.wrap = not vim.wo.wrap
end, { desc = "Toggle wrap" })

-- Buffers (group registered in mini.clue as "+Buffer")
map("n", "<leader>bs", "<cmd>FzfLua buffers<cr>", { desc = "Search buffers" })
map("n", "<leader>bb", "<cmd>buffer #<cr>", { desc = "Last buffer" })

-- Delete current buffer without closing its window/split.
local function bufdelete()
  local cur = vim.api.nvim_get_current_buf()
  local alt = vim.fn.bufnr("#")
  if alt ~= -1 and vim.api.nvim_buf_is_valid(alt) and vim.bo[alt].buflisted then
    vim.api.nvim_set_current_buf(alt)
  else
    vim.cmd("bnext")
  end
  if vim.api.nvim_get_current_buf() == cur then
    vim.cmd("enew")
  end
  pcall(vim.cmd, "bdelete " .. cur)
end
map("n", "<leader>bd", bufdelete, { desc = "Delete buffer" })
map("n", "<leader>bo", "<cmd>%bdelete|edit#|bdelete#<cr>", { desc = "Delete other buffers" })

-- Git: lazygit in a scratch terminal tab (other git maps live in mini/gitlinker)
map("n", "<leader>gg", require("config.util").lazygit, { desc = "Lazygit" })
