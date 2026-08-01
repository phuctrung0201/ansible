require("obsidian").setup({
  legacy_commands = false,
  link = {
    style = "markdown",
    format = "shortest",
  },
  workspaces = {
    { name = "wiki", path = "~/wiki" },
  },
  notes_subdir = "notes",
  new_notes_location = "notes_subdir",
  note_id_func = require("obsidian.builtin").title_id,
  note = { template = "note.md" },
  daily_notes = {
    folder = "journals",
    date_format = "%Y-%m-%d",
    alias_format = "%A, %B %-d %Y",
    template = "journal.md",
  },
  templates = {
    folder = "_templates",
    date_format = "%A, %B %-d %Y",
    time_format = "%H:%M",
  },
  picker = { name = "mini.pick" },
  frontmatter = { enabled = false },
})

vim.keymap.set("n", "<leader>ot", "<cmd>Obsidian today<cr>", { desc = "Obsidian: today" })
vim.keymap.set("n", "<leader>oy", "<cmd>Obsidian yesterday<cr>", { desc = "Obsidian: yesterday" })
vim.keymap.set("n", "<leader>on", "<cmd>Obsidian new<cr>", { desc = "Obsidian: new note" })
vim.keymap.set("n", "<leader>oN", "<cmd>Obsidian new_from_template<cr>", { desc = "Obsidian: new from template" })
vim.keymap.set("n", "<leader>op", "<cmd>Obsidian quick_switch<cr>", { desc = "Obsidian: pick note" })
vim.keymap.set("n", "<leader>os", "<cmd>Obsidian search<cr>", { desc = "Obsidian: search" })
vim.keymap.set("n", "<leader>ob", "<cmd>Obsidian backlinks<cr>", { desc = "Obsidian: backlinks" })
vim.keymap.set("n", "<leader>oi", "<cmd>Obsidian links<cr>", { desc = "Obsidian: links in note" })
vim.keymap.set("n", "<leader>or", "<cmd>Obsidian rename<cr>", { desc = "Obsidian: rename note" })
vim.keymap.set("n", "<leader>oT", "<cmd>Obsidian tags<cr>", { desc = "Obsidian: tags" })
vim.keymap.set("n", "<leader>of", "<cmd>Obsidian follow_link<cr>", { desc = "Obsidian: follow link" })
vim.keymap.set("n", "<leader>oe", "<cmd>Obsidian template<cr>", { desc = "Obsidian: insert template" })

vim.api.nvim_create_autocmd("FileType", {
  pattern = "markdown",
  callback = function(args)
    vim.keymap.set("n", "<leader>oo", "<cmd>Obsidian open<cr>", { buffer = args.buf, desc = "Obsidian: open in app" })
  end,
})
