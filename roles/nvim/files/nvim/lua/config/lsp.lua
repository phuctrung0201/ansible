require("mason").setup({
  registries = {
    "github:mason-org/mason-registry",
    "github:Crashdummyy/mason-registry",
  },
  ensure_installed = {
    "roslyn",
    "shellcheck",
    "lua-language-server",
    "bash-language-server",
    "fish-lsp",
    "vtsls",
    "gopls",
    "gofumpt",
    "goimports",
    "golangci-lint",
    "pyright",
    "ruff",
    "json-lsp",
    "stylua",
    "shfmt",
  },
})

require("fidget").setup({
  notification = {
    window = {
      normal_hl = "Comment",
      winblend = 0,
      border = "none",
    },
  },
})

vim.diagnostic.config({
  virtual_text = { spacing = 4 },
  signs = true,
  underline = true,
  update_in_insert = false,
  severity_sort = true,
  float = { border = "rounded" },
})

vim.api.nvim_create_autocmd("LspAttach", {
  group = vim.api.nvim_create_augroup("lsp_attach_keymaps", { clear = true }),
  callback = function(ev)
    local map = function(mode, lhs, rhs, desc)
      vim.keymap.set(mode, lhs, rhs, { buffer = ev.buf, desc = desc })
    end
    map("n", "gd", vim.lsp.buf.definition, "Goto definition")
    map("n", "gD", vim.lsp.buf.declaration, "Goto declaration")
    map("n", "gI", vim.lsp.buf.implementation, "Goto implementation")
    map("n", "gy", vim.lsp.buf.type_definition, "Goto type definition")
    map("n", "gr", vim.lsp.buf.references, "References")
    map("n", "gK", vim.lsp.buf.signature_help, "Signature help")
    map("n", "<leader>fs", function() MiniExtra.pickers.lsp({ scope = "document_symbol" }) end, "Find Symbol")
    map("n", "<leader>fS", function() MiniExtra.pickers.lsp({ scope = "workspace_symbol_live" }) end, "Find Symbol (workspace)")
    map("n", "<leader>cl", vim.lsp.codelens.run, "Run codelens")
    map("n", "<leader>cd", vim.diagnostic.open_float, "Line diagnostics")
  end,
})

vim.lsp.config("*", {
  capabilities = {
    textDocument = {
      completion = { completionItem = { snippetSupport = true } },
    },
  },
})

vim.lsp.config("bashls", {
  filetypes = { "sh", "zsh", "bash" },
})

vim.lsp.config("vtsls", {
  settings = {
    typescript = {
      tsserver = { maxTsServerMemory = 4096 },
    },
    javascript = {
      tsserver = { maxTsServerMemory = 4096 },
    },
  },
})

vim.lsp.config("gopls", {
  settings = {
    gopls = {
      gofumpt = true,
    },
  },
})

vim.lsp.config("sqls", { enabled = false })

vim.lsp.config("lua_ls", {
  settings = {
    Lua = {
      diagnostics = { globals = { "vim" } },
      workspace = { checkThirdParty = false },
    },
  },
})

vim.lsp.enable({
  "bashls",
  "fish_lsp",
  "vtsls",
  "gopls",
  "pyright",
  "rust_analyzer",
  "zls",
  "jsonls",
  "lua_ls",
})

require("roslyn").setup({
  extensions = {
    razor = { enabled = false },
  },
})

do
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
end
