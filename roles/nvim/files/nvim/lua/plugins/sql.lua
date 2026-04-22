return {
  {
    "neovim/nvim-lspconfig",
    opts = {
      servers = {
        sqls = { enabled = false },
      },
    },
  },

  {
    "stevearc/conform.nvim",
    optional = true,
    opts = function(_, opts)
      opts.formatters_by_ft = opts.formatters_by_ft or {}
      for _, ft in ipairs({ "sql", "mysql", "plsql" }) do
        opts.formatters_by_ft[ft] = vim.tbl_filter(function(f)
          return f ~= "sqlfluff"
        end, opts.formatters_by_ft[ft] or {})
      end
    end,
  },

  {
    "mfussenegger/nvim-lint",
    optional = true,
    opts = function(_, opts)
      opts.linters_by_ft = opts.linters_by_ft or {}
      for _, ft in ipairs({ "sql", "mysql", "plsql" }) do
        opts.linters_by_ft[ft] = vim.tbl_filter(function(l)
          return l ~= "sqlfluff"
        end, opts.linters_by_ft[ft] or {})
      end
    end,
  },

  {
    "kristijanhusak/vim-dadbod-ui",
    keys = {
      { "<leader>DD", "<cmd>DBUIToggle<CR>", desc = "Toggle DB UI" },
    },
    init = function()
      vim.g.db_ui_use_nvim_notify = true
      vim.g.db_ui_disable_mappings = 1
      vim.g.db_ui_auto_execute_table_helpers = 0
      vim.g.db_ui_execute_on_save = 0

      vim.api.nvim_create_autocmd("BufWinEnter", {
        callback = function()
          vim.schedule(function()
            if vim.b.dbui_db_key_name then
              for _, win in ipairs(vim.api.nvim_list_wins()) do
                if vim.bo[vim.api.nvim_win_get_buf(win)].filetype == "snacks_dashboard" then
                  vim.api.nvim_win_close(win, false)
                  break
                end
              end
            end
          end)
        end,
      })

      vim.api.nvim_create_autocmd("FileType", {
        pattern = { "sql", "mysql", "plsql" },
        callback = function(ev)
          local map = function(mode, l, r, desc)
            vim.keymap.set(mode, l, r, { buffer = ev.buf, desc = desc })
          end
          map({ "n", "x" }, "<leader>DS", "<Plug>(DBUI_ExecuteQuery)", "Execute query")
          map("n", "<leader>Ds", "<Plug>(DBUI_SaveQuery)", "Save query")
          map("n", "<leader>DE", "<Plug>(DBUI_EditBindParameters)", "Execute with bind params")
        end,
      })

      vim.api.nvim_create_autocmd("FileType", {
        pattern = "dbui",
        callback = function(ev)
          local map = function(l, r, desc)
            vim.keymap.set("n", l, r, { buffer = ev.buf, desc = desc })
          end
          map("o", "<Plug>(DBUI_SelectLine)", "Open/toggle")
          map("<CR>", "<Plug>(DBUI_SelectLine)", "Open/toggle")
          map("<2-LeftMouse>", "<Plug>(DBUI_SelectLine)", "Open/toggle")
          map("v", "<Plug>(DBUI_SelectLineVsplit)", "Open in vsplit")
          map("d", "<Plug>(DBUI_DeleteLine)", "Delete")
          map("D", "<Plug>(DBUI_DeleteLine)", "Delete connection")
          map("R", "<Plug>(DBUI_Redraw)", "Redraw")
          map("r", "<Plug>(DBUI_RenameLine)", "Rename")
          map("A", "<Plug>(DBUI_AddConnection)", "Add connection")
          map("H", "<Plug>(DBUI_ToggleDetails)", "Toggle details")
          map("q", "<Plug>(DBUI_Quit)", "Quit")
          map("?", "<Plug>(DBUI_ToggleHelp)", "Toggle help")
        end,
      })
    end,
  },

  {
    "folke/which-key.nvim",
    opts = {
      spec = {
        { "<leader>D", group = "database" },
        { "<leader>D", group = "database", mode = "x" },
      },
    },
  },
}
