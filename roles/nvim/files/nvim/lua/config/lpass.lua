local M = {}

local function notify(msg, level)
  vim.notify(msg, level or vim.log.levels.INFO, { title = "lpass" })
end

local function notify_result(out, ok_msg)
  if out.code == 0 then
    notify(ok_msg)
  else
    local err = vim.trim(out.stderr or "")
    notify(err ~= "" and err or "lpass failed", vim.log.levels.ERROR)
  end
end

local function lpass_async(args, cb)
  vim.system(vim.list_extend({ "lpass" }, args), { text = true }, function(out)
    vim.schedule(function() if cb then cb(out) end end)
  end)
end

local function fetch_entries()
  local out = vim.system({ "lpass", "ls" }, { text = true }):wait()
  if out.code ~= 0 then
    notify(vim.trim(out.stderr or "") ~= "" and out.stderr or "lpass ls failed", vim.log.levels.ERROR)
    return {}
  end
  local entries = {}
  for line in vim.gsplit(out.stdout, "\n", { plain = true }) do
    local id = line:match("%[id:%s*(%d+)%]")
    local name = line:match("^(.-)%s+%[id:")
    if id and name then table.insert(entries, { name = name, id = id }) end
  end
  return entries
end

local ENTRY_ACTIONS = {
  {
    label = "copy password",
    fn = function(entry)
      lpass_async({ "show", "--clip", "--password", entry.id }, function(o) notify_result(o, "password copied") end)
    end,
  },
  {
    label = "copy username",
    fn = function(entry)
      lpass_async({ "show", "--clip", "--username", entry.id }, function(o) notify_result(o, "username copied") end)
    end,
  },
  {
    label = "copy url",
    fn = function(entry)
      lpass_async({ "show", "--clip", "--url", entry.id }, function(o) notify_result(o, "url copied") end)
    end,
  },
  {
    label = "generate password",
    fn = function(entry)
      lpass_async({ "generate", "--clip", entry.id, "20" }, function(o) notify_result(o, "password generated") end)
    end,
  },
  {
    label = "edit",
    fn = function(entry)
      require("snacks").terminal({ "lpass", "edit", entry.id })
    end,
  },
  {
    label = "delete",
    fn = function(entry)
      vim.ui.input({ prompt = "Delete '" .. entry.name .. "'? (y/N) " }, function(input)
        if input and input:lower() == "y" then
          lpass_async({ "rm", entry.id }, function(o) notify_result(o, "deleted") end)
        end
      end)
    end,
  },
}

local function pick_action(entry)
  vim.ui.select(ENTRY_ACTIONS, {
    prompt = entry.name,
    format_item = function(action) return action.label end,
  }, function(action)
    if action then action.fn(entry) end
  end)
end

local function add_entry()
  vim.ui.input({ prompt = "Entry name: " }, function(name)
    if name and name ~= "" then
      require("snacks").terminal({ "lpass", "add", name })
    end
  end)
end

local function build_items()
  local items = {
    { kind = "add", text = "+ add new entry" },
  }
  for _, entry in ipairs(fetch_entries()) do
    table.insert(items, { kind = "entry", entry = entry, text = entry.name })
  end
  return items
end

function M.run()
  require("snacks").picker.pick({
    source = "lpass",
    title = "LastPass",
    items = build_items(),
    preview = "none",
    layout = { preset = "select" },
    format = function(item)
      if item.kind == "add" then
        return { { item.text, "Special" } }
      end
      return { { item.entry.name } }
    end,
    confirm = function(picker, item)
      picker:close()
      if not item then return end
      if item.kind == "add" then
        add_entry()
      else
        pick_action(item.entry)
      end
    end,
  })
end

return M
