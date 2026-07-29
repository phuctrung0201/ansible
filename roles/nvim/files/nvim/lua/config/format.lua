local formatters_by_ft = {
  lua = { "stylua" },
  go = { "gofumpt", "goimports" },
  python = { "ruff_format" },
  javascript = { "prettier" },
  typescript = { "prettier" },
  javascriptreact = { "prettier" },
  typescriptreact = { "prettier" },
  json = { "prettier" },
  rust = { "rustfmt" },
  fish = { "fish_indent" },
  sh = { "shfmt" },
}

require("conform").setup({
  formatters_by_ft = formatters_by_ft,
  format_on_save = {
    timeout_ms = 2000,
    lsp_format = "fallback",
  },
})

require("lint").linters_by_ft = {
  python = { "ruff" },
  go = { "golangcilint" },
  sh = { "shellcheck" },
}

vim.api.nvim_create_autocmd({ "BufWritePost" }, {
  group = vim.api.nvim_create_augroup("lint_on_save", { clear = true }),
  callback = function()
    require("lint").try_lint()
  end,
})

vim.keymap.set("n", "<leader>cf", function()
  require("conform").format({ async = true, lsp_format = "fallback" })
end, { desc = "Format buffer" })
