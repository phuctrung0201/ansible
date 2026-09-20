local autocmd = vim.api.nvim_create_autocmd
local augroup = vim.api.nvim_create_augroup

autocmd("TextYankPost", {
  desc = "Highlight when yanking text",
  group = augroup("highlight_yank", { clear = true }),
  callback = function()
    vim.hl.on_yank()
  end,
})

autocmd("TermOpen", {
  group = augroup("term_scrollback", { clear = true }),
  callback = function()
    vim.opt_local.scrollback = 1000
    -- No line numbers in terminal buffers. Clearing 'statuscolumn' is what does
    -- it: terminal buffers already have 'number' and 'relativenumber' off, so
    -- the global statuscolumn was the only thing still drawing numbers.
    vim.opt_local.statuscolumn = ""
  end,
})

autocmd("FileType", {
  group = augroup("disable_spell", { clear = true }),
  pattern = { "text", "plaintex", "typst", "gitcommit", "markdown" },
  callback = function()
    vim.opt_local.spell = false
  end,
})
