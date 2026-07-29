require("mini.icons").setup()
require("mini.ai").setup()
require("mini.surround").setup()
require("mini.comment").setup()
require("mini.pairs").setup()
require("mini.bufremove").setup()

require("mini.completion").setup({
  lsp_completion = {
    source_func = "omnifunc",
    auto_setup = true,
  },
  window = {
    info = { border = "rounded" },
    signature = { border = "rounded" },
  },
})
require("mini.snippets").setup()

require("fzf-lua").setup({
  "default",
  winopts = { preview = { default = "bat" } },
  files = { hidden = false, cwd_prompt = false },
})

require("mini.files").setup()
require("mini.starter").setup({
  items = {
    { name = "Find File", action = "FzfLua files", section = "Actions" },
    { name = "Recent Files", action = "FzfLua oldfiles", section = "Actions" },
    { name = "Search", action = "FzfLua live_grep_native", section = "Actions" },
    { name = "Explorer", action = "lua MiniFiles.open()", section = "Actions" },
    { name = "Wiki Today", action = "Obsidian today", section = "Wiki" },
    { name = "Wiki Search", action = "Obsidian search", section = "Wiki" },
    { name = "Git", action = "lua vim.cmd('terminal lazygit')", section = "Actions" },
    { name = "Quit", action = "qa", section = "Actions" },
  },
})

require("mini.statusline").setup()

vim.o.showtabline = 0

require("mini.clue").setup({
  triggers = {
    { mode = "n", keys = "<leader>" },
    { mode = "x", keys = "<leader>" },
    { mode = "n", keys = "g" },
    { mode = "x", keys = "g" },
    { mode = "n", keys = "z" },
    { mode = "x", keys = "z" },
    { mode = "i", keys = "<C-x>" },
    { mode = "n", keys = "[" },
    { mode = "n", keys = "]" },
    { mode = "n", keys = "'" },
    { mode = "n", keys = "`" },
    { mode = "n", keys = '"' },
    { mode = "x", keys = '"' },
    { mode = "i", keys = "<C-r>" },
    { mode = "c", keys = "<C-r>" },
  },
  clues = {
    { mode = "n", keys = "<leader>o", desc = "+Obsidian" },
    { mode = "n", keys = "<leader>d", desc = "+Database" },
    { mode = "x", keys = "<leader>d", desc = "+Database" },
    { mode = "n", keys = "<leader>c", desc = "+Code" },
    { mode = "n", keys = "<leader>f", desc = "+Find" },
    { mode = "n", keys = "<leader>g", desc = "+Git" },
    require("mini.clue").gen_clues.builtin_completion(),
    require("mini.clue").gen_clues.g(),
    require("mini.clue").gen_clues.marks(),
    require("mini.clue").gen_clues.registers(),
    require("mini.clue").gen_clues.windows(),
    require("mini.clue").gen_clues.z(),
  },
  window = {
    delay = 300,
    config = { border = "rounded" },
  },
})

vim.keymap.set("n", "<leader>ff", "<cmd>FzfLua files<cr>", { desc = "Find File" })
vim.keymap.set("n", "<leader>fg", "<cmd>FzfLua live_grep_native<cr>", { desc = "Find Text" })
vim.keymap.set("n", "<leader>fb", "<cmd>FzfLua buffers<cr>", { desc = "Find Buffer" })
vim.keymap.set("n", "<leader>fr", "<cmd>FzfLua oldfiles<cr>", { desc = "Recent Files" })
vim.keymap.set("n", "<leader>fh", "<cmd>FzfLua help_tags<cr>", { desc = "Help" })
vim.keymap.set("n", "<leader>e", "<cmd>lua MiniFiles.open()<cr>", { desc = "Explorer" })
vim.keymap.set("n", "<leader>bd", "<cmd>lua MiniBufremove.delete()<cr>", { desc = "Delete buffer" })
