return {
  {
    "lervag/wiki.vim",
    lazy = false,
    init = function()
      vim.g.wiki_root = "~/wiki"
      vim.g.wiki_filetypes = { "md" }
      vim.g.wiki_link_target_type = "md"
      vim.g.wiki_mappings_global = {
        ["<plug>(wiki-pages)"] = "<leader>wp",
      }
    end,
  },
}
