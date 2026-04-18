local autocmd = vim.api.nvim_create_autocmd
local augroup = vim.api.nvim_create_augroup

autocmd("LspAttach", {
  group = augroup("lsp_keymaps", { clear = true }),
  callback = function(ev)
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
    map("<leader>ch", vim.lsp.buf.signature_help, "Signature help")
    map("<leader>cr", vim.lsp.buf.rename, "Rename")
    map("<leader>cf", vim.diagnostic.open_float, "Diagnostic float")
    map("<leader>cF", function() vim.lsp.buf.format({ async = true }) end, "Format buffer")
    map("<leader>cs", function() Snacks.picker.lsp_symbols() end, "Document symbols")
    map("<leader>cS", function() Snacks.picker.lsp_workspace_symbols() end, "Workspace symbols")
    map("<leader>cl", function() Snacks.picker.diagnostics_buffer() end, "Diagnostic buffer list")
    map("<leader>cL", function() Snacks.picker.diagnostics() end, "Diagnostic workspace list")
    map("[d", vim.diagnostic.goto_prev, "Prev diagnostic")
    map("]d", vim.diagnostic.goto_next, "Next diagnostic")
  end,
})

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
