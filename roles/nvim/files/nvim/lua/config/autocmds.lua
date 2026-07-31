local autocmd = vim.api.nvim_create_autocmd
local augroup = vim.api.nvim_create_augroup

autocmd("TermOpen", {
  group = augroup("term_scrollback", { clear = true }),
  callback = function() vim.opt_local.scrollback = 1000 end,
})

vim.keymap.set("t", "<A-\\>", "<C-\\><C-n>", { desc = "Exit terminal mode" })

autocmd("TextYankPost", {
  group = augroup("highlight_yank", { clear = true }),
  callback = function() vim.highlight.on_yank({ higroup = "Visual", timeout = 200 }) end,
})

