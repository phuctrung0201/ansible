local autocmd = vim.api.nvim_create_autocmd
local augroup = vim.api.nvim_create_augroup

autocmd("TextYankPost", {
  group = augroup("highlight_yank", { clear = true }),
  callback = function()
    vim.highlight.on_yank()
  end,
})

autocmd("VimResized", {
  group = augroup("resize_splits", { clear = true }),
  callback = function()
    vim.cmd("tabdo wincmd =")
  end,
})

autocmd("TermOpen", {
  group = augroup("term_line_numbers", { clear = true }),
  callback = function()
    vim.opt_local.number = true
    vim.opt_local.relativenumber = true
  end,
})

autocmd("BufWritePre", {
  group = augroup("format_on_save", { clear = true }),
  pattern = "*",
  callback = function(args)
    local bo = vim.bo[args.buf]
    if not bo.modifiable or bo.readonly then
      return
    end
    local bufnr = args.buf
    local notify = vim.notify
    vim.notify = function(...) end
    pcall(function()
      -- silent!: no command-line messages; notify stub: no Snacks/LSP toast during format
      vim.cmd(string.format("silent! lua vim.lsp.buf.format({ async = false, bufnr = %d })", bufnr))
    end)
    vim.notify = notify
  end,
})

autocmd("BufReadPost", {
  group = augroup("last_cursor_position", { clear = true }),
  callback = function()
    local mark = vim.api.nvim_buf_get_mark(0, '"')
    local lcount = vim.api.nvim_buf_line_count(0)
    if mark[1] > 0 and mark[1] <= lcount then
      pcall(vim.api.nvim_win_set_cursor, 0, mark)
    end
  end,
})
