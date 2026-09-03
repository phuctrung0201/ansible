local M = {}

-- Open lazygit in a centered floating terminal that closes itself on exit.
function M.lazygit()
  local width = math.floor(vim.o.columns * 0.9)
  local height = math.floor(vim.o.lines * 0.9)
  local buf = vim.api.nvim_create_buf(false, true)
  local win = vim.api.nvim_open_win(buf, true, {
    relative = "editor",
    width = width,
    height = height,
    row = math.floor((vim.o.lines - height) / 2),
    col = math.floor((vim.o.columns - width) / 2),
    style = "minimal", -- no number column / gutter
    border = "rounded",
  })
  vim.bo[buf].bufhidden = "wipe" -- wipe the terminal buffer when the float closes
  vim.fn.jobstart({ "lazygit" }, {
    term = true,
    on_exit = function()
      vim.schedule(function()
        pcall(vim.api.nvim_win_close, win, true)
      end)
    end,
  })
  vim.cmd("startinsert")
end

-- Vault root: nearest ancestor containing `.obsidian`, else the file's dir.
local function vault_root(from)
  local dir = vim.fs.dirname(from)
  local marker = vim.fs.find(".obsidian", { path = dir, upward = true })[1]
  return marker and vim.fs.dirname(marker) or dir
end

-- Jump to a resolved file, recording the jumplist and optional heading.
local function open_file(path, fragment)
  vim.cmd("normal! m'")
  vim.cmd.edit(vim.fn.fnameescape(path))
  if fragment then
    pcall(function() require("markview.links").__to_fragment(0, fragment) end)
  end
end

-- Resolve an Obsidian note name to a file: try relative to the current file and
-- the vault root, then fall back to a recursive search by basename.
local function resolve_note(name)
  if not name:match("%.%w+$") then
    name = name .. ".md"
  end
  local base = vim.api.nvim_buf_get_name(0)
  local root = vault_root(base)
  for _, cand in ipairs({
    vim.fs.dirname(base) .. "/" .. name,
    root .. "/" .. name,
  }) do
    cand = vim.fs.normalize(cand)
    if vim.fn.filereadable(cand) ~= 0 then
      return cand
    end
  end
  return vim.fs.find(vim.fs.basename(name), { path = root, type = "file" })[1]
end

-- If the cursor sits inside a `[[wikilink]]` on the current line, return its
-- target and optional `#heading`, stripping any `|alias`.
local function wikilink_under_cursor()
  local line = vim.api.nvim_get_current_line()
  local col = vim.api.nvim_win_get_cursor(0)[2] + 1
  local init = 1
  while true do
    local s, e, inner = line:find("%[%[(.-)%]%]", init)
    if not s then
      return nil
    end
    if col >= s and col <= e then
      local target = vim.trim((inner:match("^([^|]+)") or inner))
      local name, frag = target:match("^(.-)#(.+)$")
      if name then
        return vim.trim(name), vim.trim(frag)
      end
      return target, nil
    end
    init = e + 1
  end
end

-- Follow the Markdown link under the cursor.
--
-- markview's built-in opener mishandles targets with spaces: it keeps `<...>`
-- angle brackets, doesn't decode `%20`, passes raw paths to `:edit` (so spaces
-- split into args), and resolves relative to cwd. This resolver handles Obsidian
-- `[[wikilinks]]` and local `[](...)` file links (unwrapping `<>`, decoding
-- `%20`, resolving within the vault / relative to the current file), and defers
-- headings, URLs, and everything else to markview.
function M.follow_markdown_link()
  local function markview_open()
    local ok, links = pcall(require, "markview.links")
    if ok then links.open() end
  end

  -- 1) Obsidian wikilink.
  local wl_name, wl_frag = wikilink_under_cursor()
  if wl_name then
    local resolved = resolve_note(wl_name)
    if resolved then
      open_file(resolved, wl_frag)
    else
      vim.notify("note not found: " .. wl_name, vim.log.levels.INFO)
    end
    return
  end

  -- 2) Inline link `[text](dest)`.
  local node = vim.treesitter.get_node({ ignore_injections = false })
  while node and node:type() ~= "inline_link" do
    node = node:parent()
  end
  if node == nil then
    return markview_open()
  end

  local dest
  for i = 0, node:child_count() - 1 do
    local child = node:child(i)
    if child:type() == "link_destination" then
      dest = vim.treesitter.get_node_text(child, 0)
      break
    end
  end
  if dest == nil then
    return markview_open()
  end

  local path = dest:match("^<(.*)>$") or dest
  -- Headings, URLs, and autolinks: let markview handle them.
  if path:match("^#") or path:match("^%a[%w+.-]*://") or path:match("^www%.") then
    return markview_open()
  end

  path = path:gsub("%%(%x%x)", function(h)
    return string.char(tonumber(h, 16))
  end)

  local fragment
  local file, frag = path:match("^(.-%.md)#(.+)$")
  if file then
    path, fragment = file, frag
  end

  local base = vim.api.nvim_buf_get_name(0)
  local resolved
  if vim.startswith(path, "/") then
    resolved = vim.fn.getcwd() .. path
  else
    resolved = vim.fs.dirname(base) .. "/" .. path
  end
  resolved = vim.fs.normalize(resolved)

  if vim.fn.filereadable(resolved) ~= 0 or vim.fn.isdirectory(resolved) ~= 0 then
    open_file(resolved, fragment)
  else
    vim.notify("path not found: " .. resolved, vim.log.levels.INFO)
  end
end

return M
