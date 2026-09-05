-- Markdown-only keymaps, grouped under `<leader>m`. The mini.clue group label
-- is registered buffer-locally so it only appears in Markdown buffers.
-- (Plugin specs live in lua/plugins/markdown.lua.)

-- Follow wiki links / headings via the markdown-oxide LSP (go-to-definition
-- resolves `[[links]]` and `#headings`). External URLs are handled by
-- markview's `gx`.
vim.keymap.set("n", "<CR>", vim.lsp.buf.definition,
  { buffer = true, desc = "Follow link" })
vim.keymap.set("n", "<leader>mp", "<cmd>MarkdownPreviewToggle<cr>",
  { buffer = true, desc = "Browser preview toggle" })
vim.keymap.set("n", "<leader>mr", "<cmd>Markview toggle<cr>",
  { buffer = true, desc = "Render toggle" })

vim.b.miniclue_config = {
  clues = {
    { mode = "n", keys = "<Leader>m", desc = "+Markdown" },
  },
}
