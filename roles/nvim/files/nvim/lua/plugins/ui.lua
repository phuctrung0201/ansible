return {
  {
    "Mofiqul/dracula.nvim",
    lazy = false,
    priority = 1000,
    opts = {
      transparent_bg = true,
      overrides = function(colors)
        return {
          NormalFloat = { fg = colors.fg, bg = nil },
        }
      end,
    },
    config = function(_, opts)
      require("dracula").setup(opts)
      vim.cmd.colorscheme("dracula")
    end,
  },

  {
    "echasnovski/mini.icons",
    lazy = true,
    opts = {},
    init = function()
      package.preload["nvim-web-devicons"] = function()
        require("mini.icons").mock_nvim_web_devicons()
        return package.loaded["nvim-web-devicons"]
      end
    end,
  },

  {
    "folke/which-key.nvim",
    event = "VeryLazy",
    opts = {
      spec = {
        { "<leader>c", group = "Code" },
        { "<leader>f", group = "File" },
        { "<leader>g", group = "Git" },
        { "<leader>s", group = "Search" },
      },
    },
  },

  {
    "folke/snacks.nvim",
    lazy = false,
    priority = 1000,
    keys = {
      { "<leader>e", function() Snacks.explorer() end, desc = "Toggle file explorer" },
      { "<leader><leader>", function() Snacks.picker.files() end, desc = "Find files" },
      { "<leader>sf", function() Snacks.picker.files() end, desc = "Find files" },
      { "<leader>sg", function() Snacks.picker.grep() end, desc = "Live grep" },
      { "<leader>sb", function() Snacks.picker.buffers() end, desc = "Buffers" },
      { "<leader>/", function() Snacks.picker.lines() end, desc = "Grep current buffer" },
      { "<leader>s/", function() Snacks.picker.lines() end, desc = "Grep current buffer" },
      { "<leader>sr", function() Snacks.picker.recent({ filter = { cwd = true } }) end, desc = "Recent files" },
      { "<leader>sh", function() Snacks.picker.help() end, desc = "Help tags" },
      { "<leader>sq", function() Snacks.picker.qflist() end, desc = "Quickfix" },
      { "<leader>fR", function() Snacks.rename.rename_file() end, desc = "Rename file" },
    },
    opts = {
      dashboard = {
        enabled = true,
        preset = {
          keys = {
            { icon = "󰈞 ", key = "f", desc = "Find File", action = ":lua Snacks.dashboard.pick('files')" },
            { icon = "󰈤 ", key = "n", desc = "New File", action = ":ene | startinsert" },
            { icon = "󰊄 ", key = "g", desc = "Find Text", action = ":lua Snacks.dashboard.pick('live_grep')" },
            { icon = "󰋚 ", key = "r", desc = "Recent Files", action = ":lua Snacks.dashboard.pick('oldfiles', {filter = {cwd = true}})" },
            { icon = "󰒓 ", key = "c", desc = "Config", action = ":lua Snacks.dashboard.pick('files', {cwd = vim.fn.stdpath('config')})" },
            { icon = "󰦛 ", key = "s", desc = "Restore Session", section = "session" },
            { icon = "󰒲 ", key = "L", desc = "Lazy", action = ":Lazy", enabled = package.loaded.lazy ~= nil },
            { icon = "󰩈 ", key = "q", desc = "Quit", action = ":qa" },
          },
        },
        sections = {
          { section = "header" },
          { section = "keys", gap = 1, padding = 1 },
          { section = "startup" },
        },
      },
      explorer = { enabled = true },
      input = { enabled = true },
      notifier = { enabled = true },
      rename = { enabled = true },
      picker = {
        ui_select = true,
        sources = {
          explorer = {
            win = {
              list = {
                keys = {
                  ["<c-h>"] = "toggle_hidden",
                  ["<c-i>"] = "toggle_ignored",
                },
              },
            },
          },
        },
        win = {
          input = {
            keys = {
              ["<c-h>"] = { "toggle_hidden", mode = { "i", "n" } },
              ["<c-i>"] = { "toggle_ignored", mode = { "i", "n" } },
            },
          },
        },
      },
      indent = { enabled = true },
      words = { enabled = true },
      statuscolumn = { enabled = true },
    },
  },

  {
    "mikesmithgh/kitty-scrollback.nvim",
    cmd = { "KittyScrollbackGenerateKittens", "KittyScrollbackCheckHealth", "KittyScrollbackGenerateCommandLineEditing" },
    event = { "User KittyScrollbackLaunch" },
    opts = {},
  },
}
