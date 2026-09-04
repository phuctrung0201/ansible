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

-- The link target under the cursor: the inside of an Obsidian `[[wikilink]]`
-- (minus any `|alias`) or the destination of an inline `[text](dest)`. Returns
-- nil when the cursor is not on a link.
local function link_target()
  -- Wikilink on the current line.
  local line = vim.api.nvim_get_current_line()
  local col = vim.api.nvim_win_get_cursor(0)[2] + 1
  local init = 1
  while true do
    local s, e, inner = line:find("%[%[(.-)%]%]", init)
    if not s then
      break
    end
    if col >= s and col <= e then
      return vim.trim((inner:match("^([^|]+)") or inner))
    end
    init = e + 1
  end

  -- Inline `[text](dest)` via tree-sitter.
  local node = vim.treesitter.get_node({ ignore_injections = false })
  while node and node:type() ~= "inline_link" do
    node = node:parent()
  end
  if not node then
    return nil
  end
  for i = 0, node:child_count() - 1 do
    local child = node:child(i)
    if child:type() == "link_destination" then
      return vim.treesitter.get_node_text(child, 0)
    end
  end
end

-- Resolve a link path to an existing file, relative to the current file and the
-- vault root, appending `.md` when the path has no extension.
local function resolve(path)
  local base = vim.api.nvim_buf_get_name(0)
  local root = vault_root(base)
  local variants = { path }
  if not path:match("%.%w+$") then
    variants[#variants + 1] = path .. ".md"
  end
  for _, dir in ipairs({ vim.fs.dirname(base), root }) do
    for _, name in ipairs(variants) do
      local cand = vim.fs.normalize(dir .. "/" .. name)
      if vim.fn.filereadable(cand) ~= 0 or vim.fn.isdirectory(cand) ~= 0 then
        return cand
      end
    end
  end
  return vim.fs.find(vim.fs.basename(variants[#variants]), { path = root, type = "file" })[1]
end

-- Follow the Markdown link under the cursor: open remote links (and in-file
-- headings) natively via markview; resolve everything else to a local file
-- (decoding `%20`, appending `.md`, honoring a `#heading` fragment).
function M.follow_markdown_link()
  local function markview_open()
    local ok, links = pcall(require, "markview.links")
    if ok then
      links.open()
    end
  end

  local dest = link_target()
  if not dest then
    return markview_open()
  end

  -- Remote links and in-file headings.
  if dest:match("^#") or dest:match("^%a[%w+.-]*://") or dest:match("^www%.") then
    return markview_open()
  end

  -- Local file link: decode `%20` and split off any `#heading`.
  dest = dest:gsub("%%(%x%x)", function(h)
    return string.char(tonumber(h, 16))
  end)
  local path, fragment = dest:match("^(.-)#(.+)$")
  path = path or dest

  local resolved = resolve(path)
  if not resolved then
    vim.notify("not found: " .. path, vim.log.levels.INFO)
    return
  end

  vim.cmd("normal! m'")
  vim.cmd.edit(vim.fn.fnameescape(resolved))
  if fragment then
    pcall(function()
      require("markview.links").__to_fragment(0, fragment)
    end)
  end
end

return M
