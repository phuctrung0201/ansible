local autocmd = vim.api.nvim_create_autocmd
local augroup = vim.api.nvim_create_augroup

autocmd("TextYankPost", {
  desc = "Highlight when yanking text",
  group = augroup("highlight_yank", { clear = true }),
  callback = function() vim.hl.on_yank() end,
})

autocmd("TermOpen", {
  group = augroup("term_scrollback", { clear = true }),
  callback = function() vim.opt_local.scrollback = 1000 end,
})

autocmd("FileType", {
  group = augroup("disable_spell", { clear = true }),
  pattern = { "text", "plaintex", "typst", "gitcommit", "markdown" },
  callback = function() vim.opt_local.spell = false end,
})

-- Markdown-only keymaps, grouped under `<leader>m`. The mini.clue group label
-- is registered buffer-locally so it only appears in Markdown buffers.
autocmd("FileType", {
  group = augroup("markdown_keymaps", { clear = true }),
  pattern = { "markdown", "markdown.mdx" },
  callback = function(args)
    local buf = args.buf
    -- Follow wiki links / headings via the markdown-oxide LSP (go-to-definition
    -- resolves `[[links]]` and `#headings`). External URLs are handled by
    -- markview's `gx`.
    vim.keymap.set("n", "<CR>", vim.lsp.buf.definition,
      { buffer = buf, desc = "Follow link" })
    vim.keymap.set("n", "<leader>mp", "<cmd>MarkdownPreviewToggle<cr>",
      { buffer = buf, desc = "Browser preview toggle" })
    vim.keymap.set("n", "<leader>mr", "<cmd>Markview toggle<cr>",
      { buffer = buf, desc = "Render toggle" })

    vim.b[buf].miniclue_config = {
      clues = {
        { mode = "n", keys = "<Leader>m", desc = "+Markdown" },
      },
    }
  end,
})
