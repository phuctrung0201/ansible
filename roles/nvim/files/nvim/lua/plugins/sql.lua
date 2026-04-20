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
    "kristijanhusak/vim-dadbod-ui",
    keys = {
      { "<leader>D", false },
      { "<leader>dd", "<cmd>DBUIToggle<CR>", desc = "Toggle DB UI" },
    },
    init = function()
      vim.g.db_ui_use_nvim_notify = true

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
          local map = function(l, r, desc)
            vim.keymap.set("n", l, r, { buffer = ev.buf, desc = desc })
          end
          map("<leader>ds", "<Plug>(DBUI_ExecuteQuery)", "Execute query")
          map("<leader>dS", "<Plug>(DBUI_SaveQuery)", "Save query")
          map("<leader>de", "<Plug>(DBUI_EditBindParameters)", "Execute with bind params")
        end,
      })
    end,
  },

  {
    "folke/which-key.nvim",
    opts = {
      spec = {
        { "<leader>d", group = "database" },
        { "<leader>D", group = "debug" },
      },
    },
  },
}
