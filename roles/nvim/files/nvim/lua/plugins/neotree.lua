-- File explorer (neo-tree)
return {
  "nvim-neo-tree/neo-tree.nvim",
  branch = "v3.x",
  dependencies = {
    "nvim-lua/plenary.nvim",
    "MunifTanjim/nui.nvim",
  },
  cmd = "Neotree",
  keys = {
    {
      "<leader>e",
      function()
        -- Only reveal when the current buffer is a real file on disk.
        local name = vim.api.nvim_buf_get_name(0)
        local reveal = name ~= "" and vim.uv.fs_stat(name) ~= nil
        require("neo-tree.command").execute({ toggle = true, reveal = reveal })
      end,
      desc = "File explorer",
    },
  },
  opts = {
    close_if_last_window = true,
    filesystem = {
      follow_current_file = { enabled = true },
      use_libuv_file_watcher = true,
    },
    window = {
      position = "float",
    },
  },
}
