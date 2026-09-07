-- Formatting
return {
  "stevearc/conform.nvim",
  event = { "BufWritePre" },
  cmd = { "ConformInfo" },
  keys = {
    {
      "<leader>f",
      function() require("conform").format({ async = true, lsp_format = "fallback" }) end,
      mode = { "n", "v" },
      desc = "Format buffer",
    },
  },
  opts = {
    notify_on_error = false,
    format_on_save = function(bufnr)
      if vim.b[bufnr].autoformat == false then
        return
      end
      return { timeout_ms = 500, lsp_format = "fallback" }
    end,
    formatters_by_ft = {
      lua = { "stylua" },
      go = { "gofumpt" },
      python = { "ruff_format" },
      javascript = { "prettier" },
      typescript = { "prettier" },
      javascriptreact = { "prettier" },
      typescriptreact = { "prettier" },
      json = { "prettier" },
      jsonc = { "prettier" },
      sh = { "shfmt" },
      rust = { "rustfmt" },
    },
  },
}
