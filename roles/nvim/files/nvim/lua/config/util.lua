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

return M
