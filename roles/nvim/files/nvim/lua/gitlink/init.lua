local M = {}

local function run(cmd)
  local out = vim.fn.systemlist(cmd)
  if vim.v.shell_error ~= 0 then
    return nil
  end
  return out[1]
end

local function normalize_remote(url)
  if not url then
    return nil
  end
  url = url:gsub("^git@([^:]+):", "https://%1/")
  url = url:gsub("^ssh://git@([^/]+)/", "https://%1/")
  url = url:gsub("%.git$", "")
  return url
end

local function get_selection_fragment()
  local mode = vim.fn.mode()

  if mode == "v" or mode == "V" then
    -- visual mode: selected lines
    local start = vim.fn.line("v")
    local finish = vim.fn.line(".")
    if start > finish then
      start, finish = finish, start
    end

    if start == finish then
      return "#L" .. start
    else
      return string.format("#L%d-L%d", start, finish)
    end
  end

  -- normal mode → no line fragment
  return ""
end

local function get_repo_relative_path()
  -- Get repo root
  local root = run("git rev-parse --show-toplevel")
  if not root then
    return nil
  end

  -- Get absolute path to current file
  local abs = vim.fn.expand("%:p")

  -- Make relative path
  local rel = abs:sub(#root + 2) -- +2 removes the trailing "/"
  return rel
end

function M.copy_url()
  local file = get_repo_relative_path()
  if file == "" then
    vim.notify("No file in buffer", vim.log.levels.ERROR)
    return
  end

  local remote = normalize_remote(run("git remote get-url origin 2>/dev/null"))
  if not remote then
    vim.notify("No git remote 'origin' found", vim.log.levels.ERROR)
    return
  end

  local branch = run("git rev-parse --abbrev-ref HEAD")
  if not branch then
    vim.notify("Could not resolve branch name", vim.log.levels.ERROR)
    return
  end

  local frag = get_selection_fragment()
  local url = string.format("%s/blob/%s/%s%s", remote, branch, file, frag)

  pcall(vim.fn.setreg, "+", url)
  pcall(vim.fn.setreg, '"', url)

  vim.notify("Copied: " .. url, vim.log.levels.INFO)
end

return M
