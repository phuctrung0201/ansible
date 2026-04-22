local autocmd = vim.api.nvim_create_autocmd
local augroup = vim.api.nvim_create_augroup

autocmd("FileType", {
  group = augroup("no_spell", { clear = true }),
  pattern = "*",
  callback = function() vim.opt_local.spell = false end,
})

-- LazyVim disables line numbers in terminals; override to keep them
autocmd("TermOpen", {
  group = augroup("term_line_numbers", { clear = true }),
  callback = function()
    vim.opt_local.number = true
    vim.opt_local.relativenumber = true
  end,
})
