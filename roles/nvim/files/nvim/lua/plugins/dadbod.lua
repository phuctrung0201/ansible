-- SQL: dadbod UI
return {
  "kristijanhusak/vim-dadbod-ui",
  dependencies = {
    "tpope/vim-dadbod",
    "kristijanhusak/vim-dadbod-completion",
  },
  cmd = { "DBUI", "DBUIToggle", "DBUIAddConnection", "DBUIFindBuffer" },
  keys = {
    { "<leader>du", "<cmd>DBUIToggle<cr>", desc = "Toggle DB UI" },
    { "<leader>df", "<cmd>DBUIFindBuffer<cr>", desc = "Find DB buffer" },
  },
  init = function()
    vim.g.db_ui_use_nvim_notify = true
    vim.g.db_ui_win_position = "right"
    vim.g.db_ui_disable_mappings = 1
    vim.g.db_ui_auto_execute_table_helpers = 0
    vim.g.db_ui_execute_on_save = 0

    vim.api.nvim_create_autocmd("BufWinEnter", {
      callback = function()
        vim.schedule(function()
          if vim.b.dbui_db_key_name then
            for _, win in ipairs(vim.api.nvim_list_wins()) do
              if vim.bo[vim.api.nvim_win_get_buf(win)].filetype == "ministarter" then
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
        local kmap = function(mode, l, r, desc)
          vim.keymap.set(mode, l, r, { buffer = ev.buf, desc = desc })
        end
        kmap({ "n", "x" }, "<leader>de", "<Plug>(DBUI_ExecuteQuery)", "Execute query")
        kmap("n", "<leader>dE", "<Plug>(DBUI_EditBindParameters)", "Execute with bind params")
        kmap("n", "<leader>ds", "<Plug>(DBUI_SaveQuery)", "Save query")
      end,
    })

    vim.api.nvim_create_autocmd("FileType", {
      pattern = "dbui",
      callback = function(ev)
        local kmap = function(l, r, desc)
          vim.keymap.set("n", l, r, { buffer = ev.buf, desc = desc })
        end
        kmap("o", "<Plug>(DBUI_SelectLine)", "Open/toggle")
        kmap("<CR>", "<Plug>(DBUI_SelectLine)", "Open/toggle")
        kmap("<2-LeftMouse>", "<Plug>(DBUI_SelectLine)", "Open/toggle")
        kmap("D", "<Plug>(DBUI_DeleteLine)", "Delete")
        kmap("d", "<Plug>(DBUI_DeleteLine)", "Delete connection")
        kmap("R", "<Plug>(DBUI_Redraw)", "Redraw")
        kmap("r", "<Plug>(DBUI_RenameLine)", "Rename")
        kmap("a", "<Plug>(DBUI_AddConnection)", "Add connection")
        kmap("h", "<Plug>(DBUI_ToggleDetails)", "Toggle details")
        kmap("q", "<Plug>(DBUI_Quit)", "Quit")
        kmap("?", "<Plug>(DBUI_ToggleHelp)", "Toggle help")
      end,
    })
  end,
}
