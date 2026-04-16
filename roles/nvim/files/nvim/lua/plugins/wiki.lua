return {
  {
    "lervag/wiki.vim",
    lazy = false,
    keys = {
      { "<leader>wj", "<cmd>WikiJournal<cr>", desc = "Journal today" },
      { "<leader>wi", "<cmd>WikiIndex<cr>", desc = "Wiki index" },
      { "<leader>wp", "<cmd>WikiPages<cr>", desc = "Wiki pages" },
    },
    init = function()
      vim.g.wiki_mappings_use_defaults = 0
      vim.g.wiki_root = vim.fn.expand("~/wiki")
      vim.g.wiki_filetypes = { "md" }
      vim.g.wiki_link_target_type = "md"
    end,
    config = function()
      vim.api.nvim_create_autocmd("User", {
        pattern = "WikiBufferInitialized",
        callback = function()
          local map = function(lhs, rhs, desc)
            vim.keymap.set("n", lhs, rhs, { buffer = true, desc = desc })
          end
          map("<cr>", "<cmd>WikiLinkFollow<cr>", "Follow link")
          map("<bs>", "<cmd>WikiLinkReturn<cr>", "Return from link")
          map("<leader>wa", "<cmd>WikiLinkAdd<cr>", "Wiki: insert link")
          map("<leader>wt", "<cmd>WikiLinkTransform<cr>", "Wiki: transform link")
          map("<leader>wG", "<cmd>WikiTocGenerate<cr>", "Wiki: generate TOC")
          map("<leader>wg", "<cmd>WikiTocGenerateLocal<cr>", "Wiki: generate local TOC")
        end,
      })
    end,
  },
}
