local theme = require("config.env")

require("catppuccin").setup({
  flavour = "mocha",
  transparent_background = true,
  integrations = { mini = true },
  custom_highlights = function()
    return {
      NormalFloat = { bg = "NONE" },
      FloatBorder = { bg = "NONE" },
      FloatTitle = { bg = "NONE" },
    }
  end,
})

vim.cmd.colorscheme("catppuccin")
vim.opt.fillchars = { eob = " " }

local function apply_transparent()
  vim.cmd("highlight MsgArea guibg=NONE ctermbg=NONE")
  local hl = vim.api.nvim_get_hl(0, { name = "MiniStatuslineFilename", link = false })
  hl.bg = nil
  vim.api.nvim_set_hl(0, "MiniStatuslineFilename", hl)
end
apply_transparent()
vim.api.nvim_create_autocmd("ColorScheme", { callback = apply_transparent })

vim.api.nvim_create_autocmd("VimEnter", {
  once = true,
  callback = function()
    vim.cmd("highlight MiniPickMatchCurrent guifg=" .. theme.pink)
  end,
})
