require("mini.icons").setup()
require("mini.notify").setup()
vim.notify = require("mini.notify").make_notify()
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
    { name = "Git", action = "LazyGit", section = "Actions" },
    { name = "Quit", action = "qa", section = "Actions" },
  },
})

local POWERLINE_ARROW = "\u{e0b0}"

require("mini.statusline").setup({
  content = {
    active = function()
      local mode, mode_hl = MiniStatusline.section_mode({ trunc_width = 120 })
      local kind = mode_hl:gsub("^MiniStatuslineMode", "")
      local git = MiniStatusline.section_git({ trunc_width = 40 })
      local diagnostics = MiniStatusline.section_diagnostics({ trunc_width = 75 })
      local lsp = MiniStatusline.section_lsp({ trunc_width = 75 })
      local filename = MiniStatusline.section_filename({ trunc_width = 140 })

      local dev_parts = {}
      for _, part in ipairs({ git, diagnostics, lsp }) do
        if part ~= nil and part ~= "" then table.insert(dev_parts, part) end
      end
      local devinfo = table.concat(dev_parts, " ")

      local out = { "%#" .. mode_hl .. "# " .. mode .. " " }
      if devinfo ~= "" then
        table.insert(out, "%#MiniStatuslineSep" .. kind .. "ToDev#" .. POWERLINE_ARROW)
        table.insert(out, "%#MiniStatuslineDevinfo# " .. devinfo .. " ")
        table.insert(out, "%#MiniStatuslineSepDevToEnd#" .. POWERLINE_ARROW)
      else
        table.insert(out, "%#MiniStatuslineSep" .. kind .. "ToEnd#" .. POWERLINE_ARROW)
      end
      table.insert(out, "%#MiniStatuslineFilename# " .. filename)
      table.insert(out, "%<%=")

      return table.concat(out)
    end,
  },
})

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
    { mode = "n", keys = "<leader>n", desc = "+Notifications" },
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
vim.keymap.set("n", "<leader>fa", "<cmd>FzfLua live_grep_native<cr>", { desc = "Search" })
vim.keymap.set("n", "<leader>fb", "<cmd>FzfLua buffers<cr>", { desc = "Find Buffer" })
vim.keymap.set("n", "<leader>fr", "<cmd>FzfLua oldfiles<cr>", { desc = "Recent Files" })
vim.keymap.set("n", "<leader>fh", "<cmd>FzfLua help_tags<cr>", { desc = "Help" })
vim.keymap.set("n", "<leader>e", "<cmd>lua MiniFiles.open()<cr>", { desc = "Explorer" })
vim.keymap.set("n", "<leader>bd", "<cmd>lua MiniBufremove.delete()<cr>", { desc = "Delete buffer" })

vim.keymap.set("n", "<leader>nh", function() require("mini.notify").show_history() end, { desc = "Notification history" })
vim.keymap.set("n", "<leader>nd", function() require("mini.notify").clear() end, { desc = "Dismiss notifications" })
