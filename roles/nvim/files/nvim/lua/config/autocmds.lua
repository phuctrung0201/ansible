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

autocmd("DirChanged", {
  group = augroup("bufferline_refresh", { clear = true }),
  callback = function() vim.cmd("redrawtabline") end,
})

vim.api.nvim_create_user_command("Lpass", function() require("config.lpass").run() end, {})
