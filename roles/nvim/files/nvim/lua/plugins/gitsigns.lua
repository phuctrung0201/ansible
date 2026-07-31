require("gitsigns").setup({
  -- mini.diff already renders hunk signs; use gitsigns only for blame/staging/preview
  signcolumn = false,
  numhl = false,
  linehl = false,
  current_line_blame = false,
  current_line_blame_opts = {
    delay = 300,
    virt_text_pos = "eol",
  },
})

local gs = function() return require("gitsigns") end

vim.keymap.set("n", "<leader>gb", function() gs().blame_line({ full = true }) end, { desc = "Blame line" })
vim.keymap.set("n", "<leader>gB", function() gs().toggle_current_line_blame() end, { desc = "Toggle line blame" })
vim.keymap.set("n", "<leader>gp", function() gs().preview_hunk() end, { desc = "Preview hunk" })
vim.keymap.set("n", "<leader>gs", function() gs().stage_hunk() end, { desc = "Stage hunk" })
vim.keymap.set("n", "<leader>gu", function() gs().undo_stage_hunk() end, { desc = "Undo stage hunk" })
vim.keymap.set("n", "]h", function() gs().nav_hunk("next") end, { desc = "Next hunk" })
vim.keymap.set("n", "[h", function() gs().nav_hunk("prev") end, { desc = "Prev hunk" })
