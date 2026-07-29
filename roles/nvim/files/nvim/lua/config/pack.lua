local function gh(repo, opts)
  return vim.tbl_extend("force", { src = "https://github.com/" .. repo }, opts or {})
end

vim.pack.add({
  gh("nvim-mini/mini.nvim"),
  gh("folke/flash.nvim"),
  gh("kdheepak/lazygit.nvim"),
  gh("catppuccin/nvim", { name = "catppuccin" }),
  gh("nvim-treesitter/nvim-treesitter", { version = "main" }),
  gh("mason-org/mason.nvim"),
  gh("neovim/nvim-lspconfig"),
  gh("seblyng/roslyn.nvim"),
  gh("j-hui/fidget.nvim"),
  gh("obsidian-nvim/obsidian.nvim"),
  gh("nvim-lua/plenary.nvim"),
  gh("tpope/vim-dadbod"),
  gh("kristijanhusak/vim-dadbod-ui"),
  gh("kristijanhusak/vim-dadbod-completion"),
  gh("stevearc/conform.nvim"),
  gh("mfussenegger/nvim-lint"),
})
