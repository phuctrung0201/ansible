return {
  {
    dir = vim.fn.stdpath("config") .. "/lua/gitlink",
    name = "gitlink",
    keys = {
      {
        "<leader>gl",
        function()
          require("gitlink").copy_url()
        end,
        mode = { "n", "v" },
        desc = "Copy Git remote link",
      },
    },
    cmd = { "GitLink" },
    config = function()
      vim.api.nvim_create_user_command("GitLink", function()
        require("gitlink").copy_url()
      end, { desc = "Copy git remote link for current file" })
    end,
  },

  {
    "lewis6991/gitsigns.nvim",
    event = "BufReadPost",
    opts = {
      on_attach = function(bufnr)
        local gs = require("gitsigns")
        local map = function(mode, l, r, desc)
          vim.keymap.set(mode, l, r, { buffer = bufnr, desc = desc })
        end

        map("n", "]h", function() gs.nav_hunk("next") end, "Next hunk")
        map("n", "[h", function() gs.nav_hunk("prev") end, "Prev hunk")
        map("n", "<leader>gs", gs.stage_hunk, "Stage hunk")
        map("n", "<leader>gr", gs.reset_hunk, "Reset hunk")
        map("v", "<leader>gs", function() gs.stage_hunk({ vim.fn.line("."), vim.fn.line("v") }) end, "Stage hunk")
        map("v", "<leader>gr", function() gs.reset_hunk({ vim.fn.line("."), vim.fn.line("v") }) end, "Reset hunk")
        map("n", "<leader>gS", gs.stage_buffer, "Stage buffer")
        map("n", "<leader>gR", gs.reset_buffer, "Reset buffer")
        map("n", "<leader>gu", gs.undo_stage_hunk, "Undo stage hunk")
        map("n", "<leader>gp", gs.preview_hunk, "Preview hunk")
        map("n", "<leader>gb", function() gs.blame_line({ full = true }) end, "Blame line")
        map("n", "<leader>gB", gs.toggle_current_line_blame, "Toggle line blame")
        map("n", "<leader>gd", gs.diffthis, "Diff against index")
        map("n", "<leader>gD", function() gs.diffthis("~") end, "Diff against last commit")
      end,
    },
  },
}
