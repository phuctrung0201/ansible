return {
  {
    -- Local plugin — path is relative to runtimepath
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
}
