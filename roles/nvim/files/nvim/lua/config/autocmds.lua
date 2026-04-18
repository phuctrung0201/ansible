local autocmd = vim.api.nvim_create_autocmd
local augroup = vim.api.nvim_create_augroup

autocmd("FileType", {
  group = augroup("no_spell", { clear = true }),
  pattern = "*",
  callback = function() vim.opt_local.spell = false end,
})

-- Scrollback files opened by tmux-leader: no gutter of any kind.
-- WinEnter fires after snacks applies statuscolumn; ours is registered later so it wins.
autocmd("WinEnter", {
  group = augroup("tmux_scrollback_clean", { clear = true }),
  pattern = "*tmux-leader-scrollback-*",
  callback = function()
    local o = vim.opt_local
    o.number = false
    o.relativenumber = false
    o.signcolumn = "no"
    o.foldcolumn = "0"
    o.statuscolumn = ""
  end,
})

-- LazyVim disables line numbers in terminals; override to keep them
autocmd("TermOpen", {
  group = augroup("term_line_numbers", { clear = true }),
  callback = function()
    vim.opt_local.number = true
    vim.opt_local.relativenumber = true
  end,
})

autocmd("BufNewFile", {
  group = augroup("wiki_journal_template", { clear = true }),
  pattern = vim.fn.expand("~/wiki/journal") .. "/*.md",
  callback = function()
    local template_path = vim.fn.stdpath("config") .. "/templates/journal.md"
    if vim.fn.filereadable(template_path) == 0 then
      return
    end
    local lines = vim.fn.readfile(template_path)
    for i, line in ipairs(lines) do
      lines[i] = line:gsub("%%%(date:([^)]+)%)", function(fmt)
        return vim.fn.strftime(fmt)
      end)
    end
    vim.api.nvim_buf_set_lines(0, 0, -1, false, lines)
  end,
})
