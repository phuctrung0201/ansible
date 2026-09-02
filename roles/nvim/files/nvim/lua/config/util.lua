local M = {}

-- Open lazygit in a scratch terminal tab (auto-closes on exit).
function M.lazygit()
  vim.cmd("tabnew")
  local buf = vim.api.nvim_get_current_buf()
  vim.fn.jobstart({ "lazygit" }, {
    term = true,
    cwd = vim.fn.getcwd(),
    on_exit = function()
      if vim.api.nvim_buf_is_valid(buf) then
        vim.api.nvim_buf_delete(buf, { force = true })
      end
    end,
  })
  vim.cmd("startinsert")
end

return M
