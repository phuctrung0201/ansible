local theme = require("config.env")

return {
  {
    "akinsho/bufferline.nvim",
    opts = {
      options = {
        mode = "tabs",
        always_show_bufferline = false,
        show_buffer_close_icons = false,
        show_close_icon = false,
        name_formatter = function(buf)
          if buf.tabnr then
            local ok, name = pcall(vim.api.nvim_tabpage_get_var, buf.tabnr, "tab_name")
            if ok and name and name ~= "" then return name end
          end
        end,
      },
    },
  },

  {
    "nvim-lualine/lualine.nvim",
    opts = function(_, opts)
      local cwd_branch = ""
      local function refresh_cwd_branch()
        vim.system(
          { "git", "-C", vim.fn.getcwd(), "rev-parse", "--abbrev-ref", "HEAD" },
          { text = true },
          function(out)
            cwd_branch = out.code == 0 and vim.trim(out.stdout) or ""
          end
        )
      end
      refresh_cwd_branch()
      vim.api.nvim_create_autocmd({ "DirChanged", "FocusGained" }, {
        callback = refresh_cwd_branch,
      })

      opts.sections.lualine_b = {
        {
          function()
            return cwd_branch
          end,
          cond = function()
            return cwd_branch ~= ""
          end,
          icon = "\238\130\160",
        },
      }
      opts.sections.lualine_c = {}
      opts.sections.lualine_y = {
        {
          function()
            return vim.fn.fnamemodify(vim.fn.getcwd(), ":~")
          end,
        },
      }
      opts.sections.lualine_z = {}
      opts.winbar = { lualine_x = { "filename" } }
      opts.inactive_winbar = { lualine_x = { "filename" } }
      return opts
    end,
  },

  {
    "catppuccin/nvim",
    name = "catppuccin",
    lazy = false,
    priority = 1000,
    opts = {
      flavour = "mocha",
      transparent_background = true,
      integrations = { snacks = true },
      custom_highlights = function()
        return {
          NormalFloat = { bg = "NONE" },
          FloatBorder = { bg = "NONE" },
          FloatTitle = { bg = "NONE" },
        }
      end,
    },
    config = function(_, opts)
      require("catppuccin").setup(opts)
      vim.cmd.colorscheme("catppuccin")
      vim.opt.fillchars = { eob = " " }

      local function apply_transparent()
        vim.cmd("highlight MsgArea guibg=NONE ctermbg=NONE")
      end
      apply_transparent()
      vim.api.nvim_create_autocmd("ColorScheme", { callback = apply_transparent })

      vim.api.nvim_create_autocmd("User", {
        pattern = "VeryLazy",
        once = true,
        callback = function()
          local cl = vim.api.nvim_get_hl(0, { name = "CursorLine", link = false })
          vim.api.nvim_set_hl(0, "SnacksCursorLine", cl)
          vim.cmd("highlight SnacksPickerDir   guifg=" .. theme.comment)
          vim.cmd("highlight SnacksPickerMatch guifg=" .. theme.pink)
        end,
      })
    end,
  },

  {
    "folke/which-key.nvim",
    opts = {
      spec = {
        { "<leader>w", group = "Wiki", icon = "󱗖" },
      },
    },
  },

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
