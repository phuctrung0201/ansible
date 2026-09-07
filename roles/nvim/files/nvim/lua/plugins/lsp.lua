-- LSP: nvim-lspconfig + mason + tool installer
return {
  "neovim/nvim-lspconfig",
  dependencies = {
    "mason-org/mason.nvim",
    "mason-org/mason-lspconfig.nvim",
    "WhoIsSethDaniel/mason-tool-installer.nvim",
  },
  config = function()
    local autocmd = vim.api.nvim_create_autocmd
    local augroup = vim.api.nvim_create_augroup
    local lsp_highlight_group = augroup("lsp_highlight", { clear = true })
    local lsp_detach_group = augroup("lsp_detach", { clear = true })

    autocmd("LspAttach", {
      group = augroup("lsp_attach", { clear = true }),
      callback = function(event)
        local buf = event.buf
        local lmap = function(keys, func, desc, mode)
          vim.keymap.set(mode or "n", keys, func, { buffer = buf, desc = "LSP: " .. desc })
        end

        local fzf = require("fzf-lua")
        lmap("grn", vim.lsp.buf.rename, "Rename")
        lmap("gra", vim.lsp.buf.code_action, "Goto code action", { "n", "x" })
        lmap("grD", vim.lsp.buf.declaration, "Goto declaration")
        lmap("grr", fzf.lsp_references, "Goto references")
        lmap("gri", fzf.lsp_implementations, "Goto implementation")
        lmap("grd", fzf.lsp_definitions, "Goto definition")
        lmap("grt", fzf.lsp_typedefs, "Goto type definition")

        local client = vim.lsp.get_client_by_id(event.data.client_id)
        if client and client:supports_method("textDocument/documentHighlight", buf) then
          vim.api.nvim_clear_autocmds({ group = lsp_highlight_group, buffer = buf })
          vim.api.nvim_clear_autocmds({ group = lsp_detach_group, buffer = buf })
          autocmd({ "CursorHold", "CursorHoldI" }, {
            buffer = buf,
            group = lsp_highlight_group,
            callback = vim.lsp.buf.document_highlight,
          })
          autocmd({ "CursorMoved", "CursorMovedI" }, {
            buffer = buf,
            group = lsp_highlight_group,
            callback = vim.lsp.buf.clear_references,
          })
          autocmd("LspDetach", {
            buffer = buf,
            group = lsp_detach_group,
            callback = function(ev2)
              local has_highlight_client = vim.iter(vim.lsp.get_clients({ bufnr = ev2.buf })):any(function(other)
                return other:supports_method("textDocument/documentHighlight", ev2.buf)
              end)
              if not has_highlight_client then
                vim.api.nvim_buf_call(ev2.buf, vim.lsp.buf.clear_references)
                vim.api.nvim_clear_autocmds({ group = lsp_highlight_group, buffer = ev2.buf })
              end
            end,
          })
        end

        if client and client:supports_method("textDocument/inlayHint", buf) then
          lmap("grh", function()
            vim.lsp.inlay_hint.enable(not vim.lsp.inlay_hint.is_enabled({ bufnr = buf }))
          end, "Toggle inlay hints")
        end
      end,
    })

    require("mason").setup({
      registries = {
        "github:mason-org/mason-registry",
        "github:Crashdummyy/mason-registry",
      },
    })

    require("mason-lspconfig").setup({ automatic_enable = false })

    require("mason-tool-installer").setup({
      ensure_installed = {
        -- LSP servers
        "lua-language-server",
        "bash-language-server",
        "fish-lsp",
        "vtsls",
        "gopls",
        "pyright",
        "rust-analyzer",
        "zls",
        "json-lsp",
        "roslyn",
        -- formatters / linters
        "stylua",
        "gofumpt",
        "prettier",
        "ruff",
        "shellcheck",
        "shfmt",
        "markdownlint",
      },
    })

    local servers = {
      bashls = { filetypes = { "sh", "zsh", "bash" } },
      fish_lsp = {},
      vtsls = {
        settings = {
          typescript = { tsserver = { maxTsServerMemory = 4096 } },
          javascript = { tsserver = { maxTsServerMemory = 4096 } },
        },
      },
      gopls = {
        settings = { gopls = { gofumpt = true } },
      },
      pyright = {},
      rust_analyzer = {},
      zls = {},
      jsonls = {},
      lua_ls = {
        settings = {
          Lua = {
            completion = { callSnippet = "Replace" },
          },
        },
      },
    }

    for name, cfg in pairs(servers) do
      vim.lsp.config(name, cfg)
      vim.lsp.enable(name)
    end
  end,
}
