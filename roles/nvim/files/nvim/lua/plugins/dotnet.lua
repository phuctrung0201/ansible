return {
  {
    "nvim-treesitter/nvim-treesitter",
    opts = {
      ensure_installed = { "c_sharp", "fsharp" },
    },
  },
  { "Hoffs/omnisharp-extended-lsp.nvim", lazy = true },
  {
    "mason-org/mason.nvim",
    opts = { ensure_installed = { "csharpier", "netcoredbg", "fantomas" } },
  },
  {
    "neovim/nvim-lspconfig",
    opts = {
      servers = {
        fsautocomplete = {},
        omnisharp = {
          handlers = {
            ["textDocument/definition"] = function(...)
              return require("omnisharp_extended").handler(...)
            end,
          },
          enable_roslyn_analyzers = true,
          organize_imports_on_format = true,
          enable_import_completion = true,
          use_modern_net = true,
        },
      },
    },
  },
  {
    "Nsidorenco/neotest-vstest",
  },
}
