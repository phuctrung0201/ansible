return {
  "lewis6991/gitsigns.nvim",
  event = { "BufReadPre", "BufNewFile" },
  opts = {
    on_attach = function(bufnr)
      local gitsigns = require("gitsigns")
      local function map(mode, lhs, rhs, desc)
        vim.keymap.set(mode, lhs, rhs, { buffer = bufnr, desc = desc })
      end

      map("n", "<leader>gd", gitsigns.preview_hunk_inline, "Preview hunk inline")
      map("n", "<leader>gs", gitsigns.stage_hunk, "Stage hunk")
      map("x", "<leader>gs", function()
        gitsigns.stage_hunk({ vim.fn.line("."), vim.fn.line("v") })
      end, "Stage hunk")
      map("n", "<leader>gr", gitsigns.reset_hunk, "Reset hunk")
      map("x", "<leader>gr", function()
        gitsigns.reset_hunk({ vim.fn.line("."), vim.fn.line("v") })
      end, "Reset hunk")
      map("n", "<leader>gS", gitsigns.stage_buffer, "Stage buffer")
      map("n", "<leader>gR", gitsigns.reset_buffer, "Reset buffer")
    end,
  },
}
