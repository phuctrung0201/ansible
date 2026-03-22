return {
  {
    "mason-org/mason.nvim",
    cmd = "Mason",
    opts = {
      registries = {
        "github:mason-org/mason-registry",
        "github:Crashdummyy/mason-registry",
      },
    },
  },
  {
    "mason-org/mason-lspconfig.nvim",
    event = "VeryLazy",
    dependencies = {
      "mason-org/mason.nvim",
      "neovim/nvim-lspconfig",
    },
    config = function()
      vim.lsp.config("gopls", {})
      vim.lsp.config("pyright", {})
      vim.lsp.config("vtsls", {})
      vim.lsp.config("rust_analyzer", {})
      vim.lsp.config("jsonls", {})
      vim.lsp.config("fsautocomplete", {})

      vim.lsp.config("lua_ls", {
        settings = {
          Lua = {
            workspace = { checkThirdParty = false },
            telemetry = { enable = false },
          },
        },
      })

      require("mason-lspconfig").setup({
        ensure_installed = {
          "gopls",
          "pyright",
          "vtsls",
          "rust_analyzer",
          "jsonls",
          "lua_ls",
          "fsautocomplete",
        },
        automatic_enable = true,
      })
    end,
  },
  {
    "seblyng/roslyn.nvim",
    event = "VeryLazy",
    dependencies = { "mason-org/mason.nvim" },
    opts = {},
    init = function()
      vim.lsp.config("roslyn", {
        settings = {
          ["csharp|formatting"] = {
            dotnet_organize_imports_on_format = true,
          },
          ["csharp|completion"] = {
            dotnet_show_completion_items_from_unimported_namespaces = true,
            dotnet_show_name_completion_suggestions = true,
          },
          ["csharp|code_lens"] = {
            dotnet_enable_references_code_lens = true,
          },
        },
      })
    end,
  },
}
