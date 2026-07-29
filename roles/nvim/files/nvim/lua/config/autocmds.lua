local autocmd = vim.api.nvim_create_autocmd
local augroup = vim.api.nvim_create_augroup

autocmd("TermOpen", {
  group = augroup("term_scrollback", { clear = true }),
  callback = function() vim.opt_local.scrollback = 1000 end,
})

autocmd("DirChanged", {
  group = augroup("bufferline_refresh", { clear = true }),
  callback = function() vim.cmd("redrawtabline") end,
})
