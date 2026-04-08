local autocmd = vim.api.nvim_create_autocmd
local augroup = vim.api.nvim_create_augroup

autocmd("LspAttach", {
  group = augroup("lsp_keymaps", { clear = true }),
  callback = function(ev)
    local client = vim.lsp.get_client_by_id(ev.data.client_id)
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
    map("<leader>cd", vim.diagnostic.open_float, "Diagnostic float")
    map("<leader>cl", function() Snacks.picker.diagnostics_buffer() end, "Diagnostic buffer list")
    map("<leader>cL", function() Snacks.picker.diagnostics() end, "Diagnostic workspace list")
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

autocmd("TermOpen", {
  group = augroup("term_line_numbers", { clear = true }),
  callback = function()
    -- Kitty scrollback → ansify (terminal buffer). Match raw terminal layout;
    -- global signcolumn/numbers would add gutters and shift text.
    if vim.g.ansify_pager then
      vim.o.laststatus = 0
      vim.o.ruler = false
      vim.opt_local.number = false
      vim.opt_local.relativenumber = false
      vim.opt_local.signcolumn = "no"
      vim.opt_local.wrap = false
      vim.bo.swapfile = false
      vim.bo.readonly = true
      -- In Terminal mode, `i` sends keys into the PTY; Esc often never reaches Neovim.
      -- Map Esc (and visual Bell / C-[) back to Terminal-Normal so keyboard works without the mouse.
      local buf = vim.api.nvim_get_current_buf()
      local to_normal = "<C-\\><C-n>"
      vim.keymap.set("t", "<Esc>", to_normal, { buffer = buf, silent = true })
      vim.keymap.set("t", "<C-[>", to_normal, { buffer = buf, silent = true })
      return
    end
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
