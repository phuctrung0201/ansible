local autocmd = vim.api.nvim_create_autocmd
local augroup = vim.api.nvim_create_augroup

autocmd("FileType", {
  group = augroup("no_spell", { clear = true }),
  pattern = "*",
  callback = function() vim.opt_local.spell = false end,
})

autocmd("TermOpen", {
  group = augroup("term_scrollback", { clear = true }),
  callback = function() vim.opt_local.scrollback = 1000 end,
})
