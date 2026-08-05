return {
  {
    "j-hui/fidget.nvim",
    event = "LspAttach",
    opts = {
      notification = {
        window = {
          normal_hl = "Comment",
          winblend = 0,
          border = "none",
        },
      },
    },
  },

  {
    "mason-org/mason.nvim",
    opts = {
      registries = {
        "github:mason-org/mason-registry",
        "github:Crashdummyy/mason-registry",
      },
      ensure_installed = { "roslyn", "shellcheck", "fish-lsp" },
    },
  },

  {
    "neovim/nvim-lspconfig",
    opts = {
      servers = {
        bashls = {
          filetypes = { "sh", "zsh", "bash" },
        },
        fish_lsp = {},
        vtsls = {
          settings = {
            typescript = {
              tsserver = { maxTsServerMemory = 4096 },
            },
            javascript = {
              tsserver = { maxTsServerMemory = 4096 },
            },
          },
        },
        gopls = {
          settings = {
            gopls = {
              gofumpt = true,
            },
          },
        },
        sqls = { enabled = false },
      },
    },
  },

  {
    "seblyng/roslyn.nvim",
    event = "VeryLazy",
    dependencies = { "mason-org/mason.nvim" },
    opts = {
      extensions = {
        razor = { enabled = false },
      },
    },
    init = function()
      local dotnet = vim.fn.exepath("dotnet")
      local root = dotnet ~= "" and vim.fn.fnamemodify(vim.fn.resolve(dotnet), ":h:h") .. "/libexec" or nil
      local cmd_env = {
        Configuration = vim.env.Configuration or "Debug",
        TMPDIR = vim.env.TMPDIR and vim.fn.resolve(vim.env.TMPDIR) or nil,
      }
      if root then
        cmd_env.DOTNET_ROOT = root
        cmd_env.DOTNET_ROOT_ARM64 = root
      end

      vim.lsp.config("roslyn", {
        cmd_env = cmd_env,
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
