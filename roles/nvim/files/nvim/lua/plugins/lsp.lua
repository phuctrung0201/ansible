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
    },
  },

  -- Per-server settings overrides
  {
    "neovim/nvim-lspconfig",
    opts = {
      servers = {
        bashls = {
          filetypes = { "sh", "zsh", "bash" },
        },
        gopls = {
          settings = {
            gopls = {
              completeUnimported = true,
              usePlaceholders = true,
            },
          },
        },
        pyright = {
          settings = {
            python = {
              analysis = { autoImportCompletions = true },
            },
          },
        },
        vtsls = {
          settings = {
            typescript = {
              tsserver = { maxTsServerMemory = 4096 },
              suggest = { autoImports = true },
              preferences = { includePackageJsonAutoImports = "auto" },
            },
            javascript = {
              tsserver = { maxTsServerMemory = 4096 },
              suggest = { autoImports = true },
              preferences = { includePackageJsonAutoImports = "auto" },
            },
          },
        },
        rust_analyzer = {
          settings = {
            ["rust-analyzer"] = {
              completion = { autoimport = { enable = true } },
            },
          },
        },
        lua_ls = {
          settings = {
            Lua = {
              workspace = { checkThirdParty = false },
              telemetry = { enable = false },
            },
          },
        },
      },
    },
  },

  {
    "saghen/blink.cmp",
    opts = {
      keymap = { preset = "enter" },
      completion = {
        menu = {
          draw = { treesitter = { "lsp" } },
        },
        documentation = {
          auto_show = true,
          auto_show_delay_ms = 200,
        },
      },
      cmdline = {
        completion = {
          list = { selection = { preselect = false } },
          menu = {
            auto_show = function()
              return vim.fn.getcmdtype() == ":"
            end,
          },
        },
      },
    },
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
