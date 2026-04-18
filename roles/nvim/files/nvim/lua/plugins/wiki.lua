return {
  {
    "lervag/wiki.vim",
    lazy = false,
    keys = {
      { "<leader>ww", "<cmd>WikiJournal<cr>", desc = "Journal today" },
      { "<leader>wI", "<cmd>WikiIndex<cr>", desc = "Wiki index" },
      { "<leader>wp", "<cmd>WikiPages<cr>", desc = "Wiki pages" },
      { "<leader>wo", "<cmd>WikiGraphOut<cr>", desc = "Wiki graph out" },
      { "<leader>wi", "<cmd>WikiGraphIn<cr>", desc = "Wiki graph in" },
    },
    init = function()
      vim.g.wiki_mappings_use_defaults = 0
      vim.g.wiki_root = vim.fn.expand("~/wiki")
      vim.g.wiki_filetypes = { "md" }
      vim.g.wiki_link_target_type = "md"
    end,
    config = function()
      vim.api.nvim_create_autocmd("BufNewFile", {
        group = vim.api.nvim_create_augroup("wiki_journal_template", { clear = true }),
        pattern = vim.fn.expand("~/wiki/journal") .. "/*.md",
        callback = function()
          local template_path = vim.fn.stdpath("config") .. "/templates/journal.md"
          if vim.fn.filereadable(template_path) == 0 then return end
          local lines = vim.fn.readfile(template_path)
          for i, line in ipairs(lines) do
            lines[i] = line:gsub("%%%(date:([^)]+)%)", function(fmt)
              return vim.fn.strftime(fmt)
            end)
          end
          vim.api.nvim_buf_set_lines(0, 0, -1, false, lines)
        end,
      })

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
