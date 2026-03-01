return {
  {
    "neovim/nvim-lspconfig",
    opts = {
      servers = {
        gopls = {},
        pyright = {
          settings = {
            python = {
              -- pythonPath = ".venv/bin/python",
            },
          },
        },
        vtsls = {},
        rust_analyzer = {},
        jsonls = {},
      },
    },
  },
  -- {
  --   "mason-org/mason.nvim",
  --   opts = { ensure_installed = { "csharpier", "netcoredbg", "fantomas" } },
  -- },
}
