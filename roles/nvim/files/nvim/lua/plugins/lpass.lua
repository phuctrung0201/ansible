return {
  {
    "folke/snacks.nvim",
    init = function()
      vim.api.nvim_create_user_command("LastPass", function()
        Snacks.picker.pick({
          title = "LastPass",
          format = "text",
          layout = { preset = "select" },
          finder = function()
            local lines = vim.fn.systemlist("lpass ls")
            if vim.v.shell_error ~= 0 then
              vim.notify("lpass: not logged in or unavailable", vim.log.levels.ERROR)
              return {}
            end
            local items = {}
            for _, line in ipairs(lines) do
              local name, id = line:match("^(.+) %[id: (%d+)%]$")
              if id then
                table.insert(items, { text = name, id = id })
              end
            end
            return items
          end,
          actions = {
            add_entry = function(picker)
              picker:close()
              local name = vim.fn.input("Entry name: ")
              if name ~= "" then
                Snacks.terminal("lpass add " .. vim.fn.shellescape(name))
              end
            end,
          },
          win = {
            input = {
              keys = { ["a"] = { "add_entry", mode = "n" } },
            },
          },
          confirm = function(picker, item)
            picker:close()
            Snacks.picker.pick({
              title = item.text,
              layout = { preset = "select" },
              items = {
                { text = "Copy password",    field = "password" },
                { text = "Copy username",    field = "username" },
                { text = "Copy URL",         field = "url" },
                { text = "Generate password" },
                { text = "Edit entry" },
                { text = "Delete entry" },
              },
              format = "text",
              confirm = function(picker2, action)
                picker2:close()
                if action.field then
                  local val = vim.fn.system("lpass show --" .. action.field .. " " .. item.id):gsub("\n$", "")
                  vim.fn.setreg("+", val)
                  vim.notify(action.text .. "d to clipboard", vim.log.levels.INFO)
                elseif action.text == "Generate password" then
                  local val = vim.fn.system("lpass generate " .. vim.fn.shellescape(item.text) .. " 20"):gsub("\n$", "")
                  vim.fn.setreg("+", val)
                  vim.notify("Password generated and copied to clipboard", vim.log.levels.INFO)
                elseif action.text == "Edit entry" then
                  Snacks.terminal("lpass edit " .. item.id)
                elseif action.text == "Delete entry" then
                  if vim.fn.confirm("Delete '" .. item.text .. "'?", "&Yes\n&No", 2) == 1 then
                    vim.fn.system("lpass rm " .. item.id)
                    vim.notify("Entry deleted", vim.log.levels.INFO)
                  end
                end
              end,
            })
          end,
        })
      end, {})
    end,
  },
}
