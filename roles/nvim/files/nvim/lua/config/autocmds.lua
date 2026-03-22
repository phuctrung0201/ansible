local autocmd = vim.api.nvim_create_autocmd
local augroup = vim.api.nvim_create_augroup

autocmd("LspAttach", {
  group = augroup("lsp_keymaps", { clear = true }),
  callback = function(ev)
    local map = function(keys, func, desc)
      vim.keymap.set("n", keys, func, { buffer = ev.buf, desc = desc })
    end
    map("gd", function() Snacks.picker.lsp_definitions() end, "Go to definition")
    map("gr", function() Snacks.picker.lsp_references() end, "Go to references")
    map("gI", function() Snacks.picker.lsp_implementations() end, "Go to implementation")
    map("gy", function() Snacks.picker.lsp_type_definitions() end, "Go to type definition")
    map("gD", vim.lsp.buf.declaration, "Go to declaration")
    map("K", vim.lsp.buf.hover, "Hover")
    map("<leader>ca", vim.lsp.buf.code_action, "Code action")
    map("<leader>cr", vim.lsp.buf.rename, "Rename")
    map("<leader>df", vim.diagnostic.open_float, "Float")
    map("<leader>dl", function() Snacks.picker.diagnostics_buffer() end, "Buffer list")
    map("<leader>dL", function() Snacks.picker.diagnostics() end, "Workspace list")
    map("[d", vim.diagnostic.goto_prev, "Prev diagnostic")
    map("]d", vim.diagnostic.goto_next, "Next diagnostic")
  end,
})


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
