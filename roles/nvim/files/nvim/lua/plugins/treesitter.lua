require("nvim-treesitter").setup()
require("nvim-treesitter").install({
  "lua",
  "vim",
  "vimdoc",
  "query",
  "bash",
  "markdown",
  "markdown_inline",
  "c_sharp",
  "fish",
  "go",
  "python",
  "typescript",
  "tsx",
  "rust",
  "json",
  "sql",
})

vim.api.nvim_create_autocmd("FileType", {
  pattern = "*",
  callback = function(ev)
    if vim.bo[ev.buf].buftype ~= "" then return end
    local ok = pcall(vim.treesitter.start, ev.buf)
    if ok then
      vim.wo.foldmethod = "expr"
      vim.wo.foldexpr = "v:lua.vim.treesitter.foldexpr()"
    end
  end,
})
