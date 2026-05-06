return {
  {
    "folke/snacks.nvim",
    opts = {
      dashboard = {
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
      picker = {
        sources = {
          buffers = {
            preview = "none",
          },
          explorer = {
            win = {
              list = {
                keys = {
                  ["H"] = false,
                  ["I"] = false,
                  ["<a-h>"] = "toggle_hidden",
                  ["<a-i>"] = "toggle_ignored",
                },
              },
            },
          },
        },
        win = {
          input = {
            keys = {
              ["<a-h>"] = { "toggle_hidden", mode = { "i", "n" } },
              ["<a-i>"] = { "toggle_ignored", mode = { "i", "n" } },
            },
          },
          list = {
            wo = { winhighlight = "CursorLine:CursorLine" },
          },
        },
      },
      styles = {
        notification_history = {
          wo = { wrap = true },
        },
      },
    },
  },
}
