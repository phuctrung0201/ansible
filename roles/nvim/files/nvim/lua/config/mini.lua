require("mini.icons").setup()
require("mini.statusline").setup({
  content = {
    active = function()
      local mode, mode_hl = MiniStatusline.section_mode({ trunc_width = 9999 })
      local filename = MiniStatusline.section_filename({ trunc_width = 9999 })
      local location = MiniStatusline.section_location({ trunc_width = 9999 })
      return MiniStatusline.combine_groups({
        { hl = mode_hl, strings = { mode } },
        "%<",
        { hl = "MiniStatuslineFilename", strings = { filename } },
        "%=",
        { hl = mode_hl, strings = { location } },
      })
    end,
  },
})
require("mini.notify").setup()
vim.o.laststatus = 0
vim.o.showtabline = 2
vim.o.tabline = "%!v:lua.MiniStatusline.active()"

require("mini.ai").setup()
require("mini.surround").setup()
require("mini.indentscope").setup()
require("mini.diff").setup()
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

require("mini.extra").setup()
require("mini.pick").setup()

require("mini.files").setup()
require("mini.starter").setup({
  items = {
    { name = "Find File", action = function() MiniPick.builtin.files() end, section = "Actions" },
    { name = "Recent Files", action = function() MiniExtra.pickers.oldfiles() end, section = "Actions" },
    { name = "Explorer", action = "lua MiniFiles.open()", section = "Actions" },
    { name = "Journal", action = "Obsidian today", section = "Actions" },
    { name = "Git", action = "LazyGit", section = "Actions" },
    { name = "Quit", action = "qa", section = "Actions" },
  },
})

require("mini.clue").setup({
  triggers = {
    { mode = "n", keys = "<leader>" },
    { mode = "x", keys = "<leader>" },
    { mode = "n", keys = "g" },
    { mode = "x", keys = "g" },
    { mode = "n", keys = "z" },
    { mode = "x", keys = "z" },
    { mode = "n", keys = "y" },
    { mode = "x", keys = "y" },
    { mode = "n", keys = "s" },
    { mode = "x", keys = "s" },
    { mode = "o", keys = "i" },
    { mode = "x", keys = "i" },
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
    { mode = "x", keys = "<leader>g", desc = "+Git" },
    { mode = "n", keys = "<leader>b", desc = "+Buffer" },
    { mode = "n", keys = "<leader>u", desc = "+UI" },
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

vim.keymap.set("n", "<leader>ff", function() MiniPick.builtin.files() end, { desc = "Find File" })
vim.keymap.set("n", "<leader>fg", function() MiniPick.builtin.grep_live() end, { desc = "Find Text" })
vim.keymap.set("n", "<leader>fb", function() MiniPick.builtin.buffers() end, { desc = "Find Buffer" })
vim.keymap.set("n", "<leader>fr", function() MiniExtra.pickers.oldfiles() end, { desc = "Recent Files" })
vim.keymap.set("n", "<leader>fh", function() MiniPick.builtin.help() end, { desc = "Help" })
vim.keymap.set("n", "<leader>e", "<cmd>lua MiniFiles.open()<cr>", { desc = "Explorer" })
vim.keymap.set("n", "<leader>n", function() MiniNotify.show_history() end, { desc = "Notification History" })
vim.keymap.set("n", "<leader>bd", "<cmd>lua MiniBufremove.delete()<cr>", { desc = "Delete buffer" })
vim.keymap.set("n", "<leader>bb", "<cmd>b#<cr>", { desc = "Previous buffer" })
vim.keymap.set("n", "<leader>uw", "<cmd>set wrap!<cr>", { desc = "Toggle wrap" })
