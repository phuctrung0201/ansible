return {
  {
    "ibhagwan/fzf-lua",
    dependencies = { "nvim-tree/nvim-web-devicons" },
    keys = {
      { "<leader><leader>", "<cmd>FzfLua files<cr>", desc = "Find files" },
      { "<leader>ff", "<cmd>FzfLua files<cr>", desc = "Find files" },
      { "<leader>fg", "<cmd>FzfLua live_grep<cr>", desc = "Live grep" },
      { "<leader>fb", "<cmd>FzfLua buffers<cr>", desc = "Buffers" },
      { "<leader>/", "<cmd>FzfLua grep_curbuf<cr>", desc = "Grep current buffer" },
      { "<leader>fr", "<cmd>FzfLua oldfiles<cr>", desc = "Recent files" },
      { "<leader>fh", "<cmd>FzfLua help_tags<cr>", desc = "Help tags" },
      { "<leader>fq", "<cmd>FzfLua quickfix<cr>", desc = "Quickfix" },
    },
    opts = function()
      local actions = require("fzf-lua.actions")
      return {
        files = {
          actions = {
            ["ctrl-h"] = { fn = actions.toggle_hidden, desc = "Toggle hidden" },
            ["ctrl-g"] = { fn = actions.toggle_ignore, desc = "Toggle ignored" },
          },
        },
        grep = {
          actions = {
            ["ctrl-h"] = { fn = actions.toggle_hidden, desc = "Toggle hidden" },
            ["ctrl-g"] = { fn = actions.toggle_ignore, desc = "Toggle ignored" },
          },
        },
      }
    end,
  },
}
