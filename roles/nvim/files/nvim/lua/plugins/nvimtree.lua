-- File explorer (nvim-tree)
return {
  "nvim-tree/nvim-tree.lua",
  dependencies = {
    "nvim-tree/nvim-web-devicons",
  },
  cmd = { "NvimTreeToggle", "NvimTreeFindFileToggle" },
  keys = {
    {
      "<leader>e",
      function()
        -- Only reveal when the current buffer is a real file on disk.
        local name = vim.api.nvim_buf_get_name(0)
        local reveal = name ~= "" and vim.uv.fs_stat(name) ~= nil
        require("nvim-tree.api").tree.toggle({ find_file = reveal })
      end,
      desc = "File explorer",
    },
  },
  init = function()
    -- Disable netrw so nvim-tree is the sole file explorer.
    vim.g.loaded_netrw = 1
    vim.g.loaded_netrwPlugin = 1
  end,
  opts = {
    hijack_cursor = true,
    update_focused_file = {
      enable = true,
    },
    filesystem_watchers = {
      enable = true,
    },
    actions = {
      open_file = {
        quit_on_open = false,
      },
    },
    view = {
      float = {
        enable = true,
        open_win_config = function()
          -- Wide, centered float (80% of the editor).
          local width = math.floor(vim.o.columns * 0.8)
          local height = math.floor((vim.o.lines - vim.o.cmdheight) * 0.8)
          return {
            relative = "editor",
            border = "rounded",
            width = width,
            height = height,
            row = math.floor(((vim.o.lines - vim.o.cmdheight) - height) / 2),
            col = math.floor((vim.o.columns - width) / 2),
          }
        end,
      },
    },
  },
}
